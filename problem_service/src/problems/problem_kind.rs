use contracts::SolveRequest;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum ProblemKind {
    LargestWindowInArray,
    TestProblem,
}

pub trait Problem {
    type Data: Debug;

    fn create() -> Self::Data;
    fn into_request(data: Self::Data) -> SolveRequest;
}
