use rand::{thread_rng, Rng};

pub enum Void {}

pub fn generate_random_pointer() -> *mut Void {
    let mut rng = thread_rng();
    let random_float: f64 = rng.gen();

    unsafe { std::mem::transmute(random_float) }
}
