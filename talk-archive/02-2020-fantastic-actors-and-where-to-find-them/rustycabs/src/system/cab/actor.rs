use tokio::stream::StreamExt;
use tokio::sync::mpsc;
use tokio::time::{delay_for, Duration};

use crate::math::Point;
use crate::system::{CabDriver, CabId, Dispatcher, Locator, RideClaim, RideNotification};

use super::CabMsg;

/// Cab's speed;
/// Unit: pixels per second
const CAB_SPEED: f32 = 75.0;

pub struct CabActor {
    /// Id of this cab, e.g. `3`
    id: CabId,

    /// Name of this cab's driver, e.g. `BoJack`
    driver: CabDriver,

    /// Current location of this cab, e.g.: `(666, 999)`
    position: Point,

    /// Instance of the `Dispatcher` actor, to which we'll talk when we'd be claiming rides (see: `announce_ride()`
    /// below)
    dispatcher: Dispatcher,

    /// Instance of the `Locator` actor, to which we'll be reporting our position
    locator: Locator,
}

impl CabActor {
    pub fn new(id: CabId, driver: CabDriver, position: Point, dispatcher: Dispatcher, locator: Locator) -> Self {
        Self {
            id,
            driver,
            position,
            dispatcher,
            locator,
        }
    }

    pub async fn start(mut self, mut mailbox: mpsc::UnboundedReceiver<CabMsg>) {
        self.locator.push_position(self.id, self.position);

        while let Some(msg) = mailbox.next().await {
            println!("msg: {:?}", msg);

            match msg {
                CabMsg::AnnounceRide { ride } => {
                    self.announce_ride(ride).await;
                }
            }
        }
    }

    async fn announce_ride(&mut self, ride: RideNotification) {
        let client_distance = self.position.to(ride.from).len();
        let destination_distance = client_distance + ride.from.to(ride.to).len();

        let claim = RideClaim {
            id: ride.id,
            cab_id: self.id,
            cab_driver: self.driver.clone(),
            client_pickup_in: (client_distance / CAB_SPEED) as u32,
            destination_arrival_in: (destination_distance / CAB_SPEED) as u32,
        };

        // When client requests a ride, *all* cabs receive a notification "hey, there's a ride!" and the fastest one
        // wins - to ensure it's _us_ who won, we have to check whether `claim_ride()` returned `Ok` or `Err`
        if self.dispatcher.claim_ride(claim).await.is_ok() {
            // Let's go pick-up our client
            self.drive_to(ride.from).await;

            // Having the client on-board, we might now safely transport them to the destination
            self.drive_to(ride.to).await;
        }
    }

    async fn drive_to(&mut self, point: Point) {
        loop {
            self.locator.push_position(self.id, self.position);

            let direction = point.to(self.position);

            // If we're already close to the destination, let's stop.
            //
            // We cannot check for `direction.len() == 0`, because of floating-point inaccuracies and we must use our
            // cab's speed (instead of e.g. `direction.len() <= 1`), because otherwise in the next step we could drive
            // past (!) the destination.
            //
            // Overall, I encourage you to try changing this line to `<= 1` and seeing what happens!
            if direction.len() <= CAB_SPEED {
                self.position = point;
                break;
            }

            // `direction.normalize()` gives us a direction vector where we should travel and then we multiply it by
            // `CAB_SPEED` to ensure we're faster than an average turtle
            self.position = self.position + direction.normalize() * CAB_SPEED;

            delay_for(Duration::from_secs(1)).await;
        }
    }
}