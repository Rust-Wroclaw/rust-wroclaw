use anyhow::*;
use derivative::Derivative;
use tokio::sync::oneshot;

use crate::math::Point;
use crate::system::{Cab, CabDriver, CabId, Ride, RideClaim};

#[derive(Derivative)]
#[derivative(Debug)]
pub enum DispatcherMsg {
    AddCab {
        cab: Cab,
    },

    FindDriver {
        id: CabId,

        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<Result<CabDriver>>,
    },

    RequestRide {
        from: Point,
        to: Point,

        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<Result<Ride>>,
    },

    ClaimRide {
        claim: RideClaim,

        #[derivative(Debug = "ignore")]
        tx: oneshot::Sender<Result<()>>,
    },
}