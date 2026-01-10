#![allow(dead_code)]
use anyhow::Result;
use env_logger::Env;
use problem_service::ProblemService;
#[tokio::main]

async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let _service = ProblemService::default().await;
    Ok(())
}
