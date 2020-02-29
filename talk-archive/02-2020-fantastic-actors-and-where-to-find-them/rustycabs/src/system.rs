use crate::math::Point;

pub use self::{
    cab::*,
    dispatcher::*,
    locator::*,
    ride::*,
};

mod cab;
mod dispatcher;
mod locator;
mod ride;

#[derive(Clone)]
pub struct System {
    pub dispatcher: Dispatcher,
    pub locator: Locator,
}

impl System {
    pub fn new() -> Self {
        let dispatcher = Dispatcher::new();
        let locator = Locator::new();

        dispatcher.add_cab(
            Cab::new(
                1,
                "BoJack".into(),
                Point::new(50, 50),
                dispatcher.clone(),
                locator.clone(),
            ),
        );

        dispatcher.add_cab(
            Cab::new(
                2,
                "Princess Carolyn".into(),
                Point::new(800, 300),
                dispatcher.clone(),
                locator.clone(),
            ),
        );

        dispatcher.add_cab(
            Cab::new(
                3,
                "Todd".into(),
                Point::new(500, 500),
                dispatcher.clone(),
                locator.clone(),
            ),
        );

        Self {
            dispatcher,
            locator,
        }
    }
}