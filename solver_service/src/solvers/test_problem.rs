use contracts::SolveResponse;

pub fn solve_test_problem(data: String) -> SolveResponse {
    SolveResponse::TestProblem(data)
}
