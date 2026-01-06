pub enum SolveResponse {
    LargestWindowInArray(i64),
    TestProblem(String),
}
pub enum SolveRequest {
    LargestWindowInArray { data: Vec<i64> },
    TestProblem { data: String },
}
