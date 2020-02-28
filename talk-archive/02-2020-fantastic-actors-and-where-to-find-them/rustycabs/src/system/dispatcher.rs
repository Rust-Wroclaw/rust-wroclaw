use anyhow::*;
use tokio::sync::{mpsc, oneshot};
use tokio::task;

use crate::math::Point;
use crate::system::{Cab, CabId, Ride, RideClaim};

use self::{
    actor::*,
    msg::*,
};

mod actor;
mod msg;

#[derive(Clone)]
pub struct Dispatcher {
    tx: mpsc::UnboundedSender<DispatcherMsg>,
}

impl Dispatcher {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        task::spawn(
            DispatcherActor::default()
                .start(rx)
        );

        Self { tx }
    }

    pub fn add_cab(&self, cab: Cab) {
        let _ = self.tx.send(
            DispatcherMsg::AddCab { cab }
        );
    }

    pub async fn find_driver(&self, id: CabId) -> Result<String> {
        let (tx, rx) = oneshot::channel();

        let _ = self.tx.send(
            DispatcherMsg::FindDriver { id, tx }
        );

        rx.await.unwrap()
    }

    pub async fn request_ride(&self, from: Point, to: Point) -> Result<Ride> {
        let (tx, rx) = oneshot::channel();

        let _ = self.tx.send(
            DispatcherMsg::RequestRide { from, to, tx }
        );

        rx.await.unwrap()
    }

    pub async fn claim_ride(&self, claim: RideClaim) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        let _ = self.tx.send(
            DispatcherMsg::ClaimRide { claim, tx }
        );

        rx.await.unwrap()
    }
}