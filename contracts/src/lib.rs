use wincode::{SchemaRead, SchemaWrite};

#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub enum SolveResponse {
    LargestWindowInArray(i64),
    TestProblem(String),
    SizeOfIsland(i64),
    Fault,
    BadData,
}
#[derive(Clone, Debug, PartialEq, SchemaWrite, SchemaRead)]
pub enum SolveRequest {
    LargestWindowInArray { data: Vec<i64> },
    TestProblem { data: String },
    SizeOfIsland { data: Vec<Vec<bool>> },
    UnimplementedProblem { data: String },
}
