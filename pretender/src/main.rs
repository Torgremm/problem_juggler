mod client;
mod problem_client;
mod user_client;

use env_logger::Env;
fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    println!("Hello, world!");
}
