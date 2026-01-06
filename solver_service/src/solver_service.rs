pub struct SolverService;
use crate::solvers::{
    largest_window_in_array::solve_largest_window_in_array, test_problem::solve_test_problem,
};
use contracts::{SolveRequest, SolveResponse};

impl SolverService {
    fn solve(req: SolveRequest) -> SolveResponse {
        match_and_solve(req)
    }
}

fn match_and_solve(req: SolveRequest) -> SolveResponse {
    match req {
        SolveRequest::LargestWindowInArray { data } => solve_largest_window_in_array(data),
        SolveRequest::TestProblem { data } => solve_test_problem(data),
    }
}
