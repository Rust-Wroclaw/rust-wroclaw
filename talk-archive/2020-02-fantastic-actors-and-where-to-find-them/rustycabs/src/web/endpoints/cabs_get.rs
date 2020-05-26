use std::convert::Infallible;

use serde::Serialize;
use warp::Reply;
use warp::reply::json;

use crate::math::Point;
use crate::system::{CabDriver, System};

#[derive(Serialize)]
pub struct Cab {
    pub id: usize,
    pub driver: CabDriver,
    pub position: Point,
}

pub async fn get_cabs(sys: System) -> Result<impl Reply, Infallible> {
    let mut cabs = Vec::new();

    for (id, position) in sys.locator.find_positions().await {
        let driver = sys.dispatcher
            .find_driver(id).await
            .unwrap();

        cabs.push(Cab {
            id,
            driver,
            position,
        });
    }

    Ok(json(&cabs))
}