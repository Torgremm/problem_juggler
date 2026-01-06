use anyhow::Result;
use async_trait::async_trait;
use contracts::SolveRequest;
use contracts::SolveResponse;

#[async_trait]
pub trait SolverClient: Send + Sync {
    async fn solve(&self, request: SolveRequest) -> Result<SolveResponse>;
}
