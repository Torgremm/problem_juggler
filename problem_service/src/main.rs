#![allow(dead_code)]
use anyhow::Result;

mod interface;
mod problem_handler;
mod problem_service;
mod problems;
use crate::problem_service::ProblemService;
use env_logger::Env;
#[tokio::main]

async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let _service = ProblemService::default().await;
    Ok(())
}
