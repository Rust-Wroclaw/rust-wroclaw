use derivative::Derivative;

use crate::system::RideNotification;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum CabMsg {
    AnnounceRide {
        ride: RideNotification,
    },
}