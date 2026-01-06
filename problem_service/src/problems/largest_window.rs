use rand::Rng;

use crate::problems::problem_kind::Problem;
use contracts::SolveRequest;

pub struct LargestWindow {
    id: Option<i64>,
    data: String,
    answer: i64,
}
impl Problem for LargestWindow {
    type Data = Vec<i64>;
    fn create() -> Vec<i64> {
        let mut rng = rand::rng();

        let mut data = Vec::new();
        for _ in 0..20 {
            data.push(rng.random_range(-20..20));
        }
        data
    }

    fn into_request(data: Self::Data) -> SolveRequest {
        SolveRequest::LargestWindowInArray { data }
    }
}
