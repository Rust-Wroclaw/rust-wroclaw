use tokio::sync::{mpsc, oneshot};
use tokio::task;

use crate::math::Point;
use crate::system::CabId;

pub use self::{
    actor::*,
    msg::*,
};

mod actor;
mod msg;

#[derive(Clone)]
pub struct Locator {
    tx: mpsc::UnboundedSender<LocatorMsg>,
}

impl Locator {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        task::spawn(
            LocatorActor::default()
                .start(rx)
        );

        Self { tx }
    }

    pub async fn find_positions(&self) -> Vec<(CabId, Point)> {
        let (tx, rx) = oneshot::channel();

        let _ = self.tx.send(
            LocatorMsg::FindPositions { tx }
        );

        rx.await.unwrap()
    }

    pub fn push_position(&self, id: CabId, position: Point) {
        let _ = self.tx.send(
            LocatorMsg::PushPosition { id, position }
        );
    }
}
