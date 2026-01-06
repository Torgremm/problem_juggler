#![allow(dead_code)]
use anyhow::Result;

mod interface;
mod problem_handler;
mod problem_service;
mod problems;
mod test_template;
use crate::problem_service::ProblemService;
#[tokio::main]
async fn main() -> Result<()> {
    let _service = ProblemService::default().await;
    Ok(())
}
