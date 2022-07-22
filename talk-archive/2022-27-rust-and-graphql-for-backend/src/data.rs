use crate::graphql::{Brand, Car, User};

pub(crate) type Users = Vec<User>;
pub(crate) type Cars = Vec<Car>;

pub(crate) fn users() -> Users {
    vec![
        User {
            id: 0,
            name: "Json".to_string(),
            surname: "Bourne".to_string(),
            age: 19,
            car_id: 4,
        },
        User {
            id: 1,
            name: "Adam".to_string(),
            surname: "Adams".to_string(),
            age: 28,
            car_id: 3,
        },
        User {
            id: 2,
            name: "Cameron".to_string(),
            surname: "Nelson".to_string(),
            age: 71,
            car_id: 2,
        },
        User {
            id: 3,
            name: "Dawn".to_string(),
            surname: "Lauer".to_string(),
            age: 21,
            car_id: 1,
        },
        User {
            id: 4,
            name: "Billy".to_string(),
            surname: "Blaser".to_string(),
            age: 66,
            car_id: 0,
        },
    ]
}

pub(crate) fn cars() -> Cars {
    vec![
        Car {
            id: 0,
            brand: Brand::Ford,
            model: "Focus".to_string(),
            year: 2015,
            owner_id: 4,
        },
        Car {
            id: 1,
            brand: Brand::Fiat,
            model: "Stilo".to_string(),
            year: 2001,
            owner_id: 3,
        },
        Car {
            id: 2,
            brand: Brand::Opel,
            model: "Astra".to_string(),
            year: 2011,
            owner_id: 2,
        },
        Car {
            id: 3,
            brand: Brand::Ford,
            model: "Mondeo".to_string(),
            year: 2018,
            owner_id: 1,
        },
        Car {
            id: 4,
            brand: Brand::Fiat,
            model: "Panda".to_string(),
            year: 2005,
            owner_id: 0,
        },
    ]
}
