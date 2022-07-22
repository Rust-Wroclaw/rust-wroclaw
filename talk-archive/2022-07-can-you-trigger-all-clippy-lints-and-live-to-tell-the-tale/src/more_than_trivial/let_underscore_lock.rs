use std::sync::Mutex;

pub fn get_max_value(x: &Mutex<i32>) -> i32 {
    let _ = x.lock().unwrap();

    // Should be changed to
    // let _lock = x.lock().unwrap();

    // TODO: implement this method

    0
}
