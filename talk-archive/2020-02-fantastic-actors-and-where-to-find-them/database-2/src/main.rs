use std::collections::HashMap;

use tokio::stream::StreamExt;
use tokio::sync::{mpsc, oneshot};
use tokio::task;

// ----- //

pub enum DatabaseMsg {
    Put {
        key: String,
        value: String,
    },

    Get {
        key: String,
        tx: oneshot::Sender<Option<String>>,
    },

    List {
        tx: oneshot::Sender<Vec<(String, String)>>,
    },
}

// ----- //

pub struct DatabaseActor {
    items: HashMap<String, String>,
}

impl DatabaseActor {
    pub fn new() -> Self {
        Self {
            items: Default::default(),
        }
    }

    pub async fn start(mut self, mut mailbox: mpsc::UnboundedReceiver<DatabaseMsg>) {
        use DatabaseMsg::*;

        while let Some(msg) = mailbox.next().await {
            match msg {
                Put { key, value } => {
                    self.items.insert(key, value);
                }

                Get { key, tx } => {
                    let value = self.items
                        .get(&key)
                        .cloned();

                    let _ = tx.send(value);
                }

                List { tx } => {
                    let items = self.items
                        .iter()
                        .map(|(key, value)| {
                            (key.clone(), value.clone())
                        })
                        .collect();

                    let _ = tx.send(items);
                }
            }
        }
    }
}

// ----- //

#[derive(Clone)]
pub struct Database {
    tx: mpsc::UnboundedSender<DatabaseMsg>,
}

impl Database {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        task::spawn(
            DatabaseActor::new()
                .start(rx)
        );

        Self { tx }
    }

    pub fn put(&self, key: impl Into<String>, value: impl Into<String>) {
        let _ = self.tx.send(
            DatabaseMsg::Put {
                key: key.into(),
                value: value.into(),
            }
        );
    }

    pub async fn get(&self, key: impl Into<String>) -> Option<String> {
        let (tx, rx) = oneshot::channel();

        let _ = self.tx.send(
            DatabaseMsg::Get {
                key: key.into(),
                tx,
            }
        );

        rx.await.unwrap()
    }

    pub async fn list(&self) -> Vec<(String, String)> {
        let (tx, rx) = oneshot::channel();

        let _ = self.tx.send(
            DatabaseMsg::List { tx }
        );

        rx.await.unwrap()
    }
}

// ----- //

#[tokio::main]
async fn main() {
    let db = Database::new();

    // ----- //

    db.put("pizza hut", "22 536 36 36");
    db.put("telepizza", "71 321 39 50");

    let db2 = db.clone();

    task::spawn(async move {
        db2.put("ozima", "71 338 85 85");
    });

    // ----- //

    println!("get(\"telepizza\") = {:?}", db.get("telepizza").await);
    println!("list() = {:?}", db.list().await);
}
