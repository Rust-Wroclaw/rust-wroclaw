use derivative::Derivative;
use tokio::sync::oneshot;

use crate::math::Point;
use crate::system::CabId;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum LocatorMsg {
    FindPositions {
        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<Vec<(CabId, Point)>>,
    },

    PushPosition {
        id: CabId,
        position: Point,
    },
}