use crate::data::{self, Cars, Users};
use async_graphql::*;
use async_stream::stream;
use futures::stream::Stream;
use tokio::sync::broadcast;

pub type GqlSchema = Schema<Query, Mutation, Subscription>;

pub fn schema() -> GqlSchema {
    let (tx, _) = broadcast::channel(1);
    Schema::build(
        Query,
        Mutation {
            users_tx: tx.clone(),
        },
        Subscription { users_tx: tx },
    )
    .data(data::users())
    .data(data::cars())
    .finish()
}

/// The root query type
#[derive(Description)]
pub struct Query;

#[Object(use_type_description)] // to get the doc from Query type
impl Query {
    /// Get all users registered in the service
    async fn users<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "Query at most this amount of users", default = 15)] limit: usize,
    ) -> Result<&'ctx [User]> {
        let users = ctx.data::<Users>()?;
        let limit = std::cmp::min(limit, users.len());
        Ok(&users[..limit])
    }

    /// Get all cars registered in the service
    async fn cars<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&'ctx [Car]> {
        let cars = ctx.data::<Cars>()?;
        Ok(&cars[..])
    }
}

pub struct Mutation {
    users_tx: broadcast::Sender<User>,
}

#[Object]
impl Mutation {
    /// Notify all listeners that the user needs help
    async fn call_for_help<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        #[graphql(desc = "Id of the user calling for help")] user_id: usize,
    ) -> Result<&'ctx User> {
        let users = ctx.data::<Users>()?;
        let user = users.iter().find(|u| u.id == user_id).ok_or("Not found")?;

        self.users_tx.send(user.clone())?;
        Ok(user)
    }
}

pub struct Subscription {
    users_tx: broadcast::Sender<User>,
}

#[Subscription]
impl Subscription {
    /// Listen to the users' help calls
    async fn subscribe_for_help_calls(&self) -> impl Stream<Item = User> {
        let mut users_rx = self.users_tx.subscribe();
        stream! {
            while let Ok(user) = users_rx.recv().await {
                yield user;
            }
        }
    }
}

/// Representation of the user
#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct User {
    /// User's id
    pub id: usize,
    /// User's name
    pub name: String,
    /// User's surname
    pub surname: String,
    /// User's age
    pub age: u32,
    /// User's car id
    #[graphql(skip)]
    pub car_id: usize,
}

#[ComplexObject]
impl User {
    /// User's car
    async fn car<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&'ctx Car> {
        let cars = ctx.data::<Cars>()?;
        cars.iter()
            .find(|c| c.id == self.car_id)
            .ok_or_else(|| "Not found".into())
    }
}

/// All the car brands supported by service
#[derive(Enum, Description, Eq, PartialEq, Clone, Copy)]
pub enum Brand {
    Fiat,
    Ford,
    Opel,
}

/// Representation of the car
#[derive(Description, SimpleObject)]
#[graphql(complex)]
pub struct Car {
    /// Car's id
    pub id: usize,
    /// Car's brand
    pub brand: Brand,
    /// Car's model
    pub model: String,
    /// Car's production year
    pub year: u32,
    /// Car's owner id
    #[graphql(skip)]
    pub owner_id: usize,
}

#[ComplexObject]
impl Car {
    /// Car's owner
    async fn owner<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&'ctx User> {
        let users = ctx.data::<Users>()?;
        users
            .iter()
            .find(|u| u.id == self.owner_id)
            .ok_or_else(|| "Not found".into())
    }
}
