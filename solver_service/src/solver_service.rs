pub struct SolverService;
use crate::solvers::*;
use contracts::{SolveRequest, SolveResponse};
impl SolverService {
    fn solve(req: SolveRequest) -> SolveResponse {
        log::debug!("received request for: {:?}", req);
        match_and_solve(req)
    }
}

fn match_and_solve(req: SolveRequest) -> SolveResponse {
    match req {
        SolveRequest::LargestWindowInArray { data } => solve_largest_window_in_array(data),
        SolveRequest::TestProblem { data } => solve_test_problem(data),
        SolveRequest::SizeOfIsland { data } => solve_size_of_island(data),
        _ => {
            log::error!("Unimplemented problem request");
            SolveResponse::Fault
        }
    }
}
