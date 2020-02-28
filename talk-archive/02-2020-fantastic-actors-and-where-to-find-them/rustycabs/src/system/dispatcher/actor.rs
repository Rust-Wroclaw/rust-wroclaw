use std::collections::HashMap;

use anyhow::*;
use tokio::stream::StreamExt;
use tokio::sync::{mpsc, oneshot};

use crate::math::Point;
use crate::system::{Cab, CabDriver, CabId, Ride, RideClaim, RideId, RideNotification};

use super::DispatcherMsg;

#[derive(Default)]
pub struct DispatcherActor {
    /// Map of all the cabs we're aware of
    cabs: HashMap<CabId, Cab>,

    /// Map of all the rides that are yet-to-be-confirmed by any driver.
    ///
    /// The spooky-looking `oneshot::Sender<...>` part is actually just a sender of the `DispatcherMsg::FindDriver`
    /// message.
    ///
    /// It's been modeled this way, so that we can have more than one pending ride - otherwise we'd be able to handle
    /// one client only.
    pending_rides: HashMap<RideId, oneshot::Sender<Result<Ride>>>,

    /// Id of the latest ride, used to build an auto-incrementing counter
    latest_ride_id: RideId,
}

impl DispatcherActor {
    pub async fn start(mut self, mut mailbox: mpsc::UnboundedReceiver<DispatcherMsg>) {
        use DispatcherMsg::*;

        while let Some(msg) = mailbox.next().await {
            println!("msg: {:?}", msg);

            match msg {
                AddCab { cab } => {
                    self.add_cab(cab);
                }

                FindDriver { id, tx } => {
                    let _ = tx.send(
                        self.find_driver(id)
                    );
                }

                RequestRide { from, to, tx } => {
                    self.request_ride(from, to, tx);
                }

                ClaimRide { claim, tx } => {
                    let _ = tx.send(
                        self.claim_ride(claim)
                    );
                }
            }
        }
    }

    fn add_cab(&mut self, cab: Cab) {
        self.cabs.insert(cab.id(), cab);
    }

    fn find_driver(&self, id: CabId) -> Result<CabDriver> {
        let cab = self.cabs
            .get(&id)
            .ok_or_else(|| anyhow!("No such driver exists"))?;

        Ok(cab.driver())
    }

    fn request_ride(&mut self, from: Point, to: Point, tx: oneshot::Sender<Result<Ride>>) {
        self.latest_ride_id += 1;

        let ride = RideNotification {
            id: self.latest_ride_id,
            from,
            to,
        };

        for cab in self.cabs.values() {
            cab.announce_ride(ride);
        }

        self.pending_rides.insert(ride.id, tx);
    }

    fn claim_ride(&mut self, claim: RideClaim) -> Result<()> {
        let responder = self.pending_rides
            .remove(&claim.id)
            .ok_or_else(|| anyhow!("No such ride exists"))?;

        let _ = responder.send(Ok(Ride {
            id: claim.id,
            driver: claim.cab_driver,
            client_pickup_in: claim.client_pickup_in,
            destination_arrival_in: claim.destination_arrival_in,
        }));

        Ok(())
    }
}