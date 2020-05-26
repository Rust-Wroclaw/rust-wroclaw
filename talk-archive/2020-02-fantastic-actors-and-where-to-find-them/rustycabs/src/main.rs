mod math;
mod system;
mod web;

#[tokio::main]
async fn main() {
    println!("Starting rusty-cabs");

    let system = system::System::new();

    web::start(system).await;
}
