use crate::math::Point;
use crate::system::{CabDriver, CabId};

pub type RideId = usize;

#[derive(Clone, Debug)]
pub struct Ride {
    pub id: RideId,

    /// Name of the cab's driver that'll pick-up the client
    pub driver: CabDriver,

    /// Estimated time when the cab will arrive to pick-up the client;
    /// Unit: seconds
    pub client_pickup_in: u32,

    /// Estimated time when the cab should arrive at the destination;
    /// Unit: seconds
    pub destination_arrival_in: u32,
}

/// Models a notification that's sent from `Dispatcher` to each `Cab` actor, when dispatcher's announcing that a new
/// ride's available.
///
/// The first `Cab` to respond to this notification gets to drive the client.
#[derive(Copy, Clone, Debug)]
pub struct RideNotification {
    pub id: RideId,
    pub from: Point,
    pub to: Point,
}

/// Models a notification that's sent from `Cab` to `Dispatcher` - it's sent when `Cab` decides that it'll drive the
/// client.
#[derive(Clone, Debug)]
pub struct RideClaim {
    pub id: RideId,
    pub cab_id: CabId,
    pub cab_driver: CabDriver,
    pub client_pickup_in: u32,
    pub destination_arrival_in: u32,
}