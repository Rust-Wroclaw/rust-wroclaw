#![allow(unused)]

use std::sync::{Arc, Mutex};

pub struct EcsSystem {
    pub ages: Vec<u32>,
    pub names: Vec<String>,
}

impl EcsSystem {
    fn oldest_person(&self) -> Option<String> {
        let (idx, _) =
            self.ages.iter().enumerate().max_by_key(|(_, age)| *age)?;

        Some(self.names[idx].clone())
    }
}

// This example is a little convoluted, because the lint actually doesn't trigger
// if the mutex is taken as a reference in function arguments.
pub struct Example {
    ecs_system: Mutex<EcsSystem>,
}

impl Example {
    pub fn get_oldest_person(&self) {
        if let Ok(_ecs) = self.ecs_system.lock() {
            // do something
        } else {
            self.ecs_system.lock();
        }
    }
}

#[test]
fn will_this_deadlock() {
    let ecs_system = EcsSystem {
        ages: vec![1, 2, 3],
        names: vec!["a", "b", "c"]
            .into_iter()
            .map(|s| s.to_string())
            .collect(),
    };

    let ecs_system = Mutex::new(ecs_system);

    let example = Arc::new(Example { ecs_system });

    {
        let example = example.clone();

        let _ = std::thread::spawn(move || {
            let _lock = example.ecs_system.lock();
            panic!("Poison the lock!")
        })
        .join();
    }

    assert!(example.ecs_system.is_poisoned());

    // example.ecs_system.lock()
    // let _lock = example.ecs_system.lock();

    // Spoiler warning: it deadlocks
    let oldest_person = example.get_oldest_person();

    panic!("{oldest_person:?}");
}
