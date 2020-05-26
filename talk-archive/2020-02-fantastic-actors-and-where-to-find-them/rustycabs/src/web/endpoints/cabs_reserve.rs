use std::convert::Infallible;

use anyhow::*;
use serde::{Deserialize, Serialize};
use warp::Reply;
use warp::reply::json;

use crate::math::Point;
use crate::system::{Ride, System};

#[derive(Deserialize)]
pub struct ReserveCabRequest {
    pub from_x: i32,
    pub from_y: i32,
    pub to_x: i32,
    pub to_y: i32,
}

#[derive(Serialize)]
pub struct ReserveCabReply {
    pub id: usize,
    pub driver: String,
    pub pickup_in: u32,
    pub destination_in: u32,
}

pub async fn reserve_cab(req: ReserveCabRequest, sys: System) -> Result<impl Reply, Infallible> {
    let from = Point::new(req.from_x, req.from_y);
    let to = Point::new(req.to_x, req.to_y);

    match sys.dispatcher.request_ride(from, to).await {
        Ok(Ride { id, driver, client_pickup_in, destination_arrival_in }) => {
            Ok(json(&ReserveCabReply {
                id,
                driver,
                pickup_in: client_pickup_in,
                destination_in: destination_arrival_in,
            }))
        }

        Err(err) => {
            panic!("{:?}", err);
        }
    }
}