use warp::Filter;
use warp::http::Method;

use crate::system::System;

use self::endpoints::*;

mod endpoints;

pub async fn start(sys: System) {
    let sys = warp::any().map(move || sys.clone());

    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST]);

    // GET /cabs
    let cabs = warp::get()
        .and(warp::path("cabs"))
        .and(sys.clone())
        .and_then(get_cabs);

    // POST /cabs/reserve
    let cabs_reserve = warp::post()
        .and(warp::path!("cabs" / "reserve"))
        .and(warp::query())
        .and(sys.clone())
        .and_then(reserve_cab);

    let routes = cabs
        .or(cabs_reserve)
        .with(cors);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 1337))
        .await;
}
