use std::collections::HashMap;

use tokio::stream::StreamExt;
use tokio::sync::mpsc;

use crate::math::Point;
use crate::system::CabId;

use super::LocatorMsg;

#[derive(Default)]
pub struct LocatorActor {
    positions: HashMap<CabId, Point>,
}

impl LocatorActor {
    pub async fn start(mut self, mut mailbox: mpsc::UnboundedReceiver<LocatorMsg>) {
        use LocatorMsg::*;

        while let Some(msg) = mailbox.next().await {
            println!("msg: {:?}", msg);

            match msg {
                FindPositions { tx } => {
                    let _ = tx.send(
                        self.positions
                            .iter()
                            .map(|(&id, &position)| (id, position))
                            .collect()
                    );
                }

                PushPosition { id, position } => {
                    self.push_position(id, position);
                }
            }
        }
    }

    fn push_position(&mut self, id: CabId, position: Point) {
        self.positions.insert(id, position);
    }
}