use derivative::Derivative;
use tokio::sync::mpsc;
use tokio::task;

use crate::math::Point;
use crate::system::{Dispatcher, Locator, RideNotification};

use self::{
    actor::*,
    msg::*,
};

pub type CabId = usize;
pub type CabDriver = String;

mod actor;
mod msg;

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct Cab {
    id: CabId,
    driver: CabDriver,

    #[derivative(Debug = "ignore")]
    tx: mpsc::UnboundedSender<CabMsg>,
}

impl Cab {
    pub fn new(id: CabId, driver: CabDriver, position: Point, dispatcher: Dispatcher, locator: Locator) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        task::spawn(
            CabActor::new(id, driver.clone(), position, dispatcher, locator)
                .start(rx)
        );

        Self { id, driver, tx }
    }

    pub fn id(&self) -> CabId {
        self.id
    }

    pub fn driver(&self) -> CabDriver {
        self.driver.clone()
    }

    pub fn announce_ride(&self, ride: RideNotification) {
        let _ = self.tx.send(
            CabMsg::AnnounceRide { ride }
        );
    }
}