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

#[tokio::main]
async fn main() {
    let (db_tx, db_rx) = mpsc::unbounded_channel();

    task::spawn(
        DatabaseActor::new()
            .start(db_rx)
    );

    // ----- //

    let _ = db_tx.send(DatabaseMsg::Put {
        key: "pizza hut".into(),
        value: "22 536 36 36".into(),
    });

    let _ = db_tx.send(DatabaseMsg::Put {
        key: "telepizza".into(),
        value: "71 321 39 50".into(),
    });

    let db_tx2 = db_tx.clone();

    task::spawn(async move {
        let _ = db_tx2.send(DatabaseMsg::Put {
            key: "ozima".into(),
            value: "71 338 85 85".into(),
        });
    });

    // ----- //

    {
        let (req_tx, req_rx) = oneshot::channel();

        let _ = db_tx.send(DatabaseMsg::Get {
            key: "telepizza".into(),
            tx: req_tx,
        });

        println!("get(\"telepizza\") = {:?}", req_rx.await.unwrap());
    }

    // ----- //

    {
        let (req_tx, req_rx) = oneshot::channel();

        let _ = db_tx.send(DatabaseMsg::List {
            tx: req_tx,
        });

        println!("list() = {:?}", req_rx.await.unwrap());
    }
}
