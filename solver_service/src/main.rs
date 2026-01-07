#![allow(dead_code)]
use env_logger::Env;
mod solver_service;
mod solvers;
fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    println!("Hello, world!");
}
