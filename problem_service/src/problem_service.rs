use std::pin::Pin;

use crate::problem_handler::ProblemRepository;
use crate::problem_handler::ProblemRow;
use crate::problems::problem_kind::Problem;
use crate::test_template::Test;
use anyhow::Result;

pub struct ProblemService {
    repo: ProblemRepository,
}

impl ProblemService {
    pub async fn default() -> Self {
        Self {
            repo: ProblemRepository::default().await,
        }
    }
}

impl ProblemService {
    pub async fn get<P: Problem>(&self) -> Result<ProblemRow> {
        let data = P::create();
        let data_string = format!("{:?}", data);
        let _request = P::into_request(data);
        let answer = 0; //self.solver.solve(request).await?;
        let id = self.repo.insert((data_string.clone(), answer)).await?;
        Ok(ProblemRow::new(id.try_into()?, answer, data_string))
    }
    pub async fn validate(&self, id: i64, answer: i64) -> Result<bool> {
        let problem = self.query(id).await?;
        Ok(problem.validate(answer))
    }
    pub async fn query(&self, id: i64) -> Result<ProblemRow> {
        let problem = self.repo.get(id).await?;
        Ok(problem)
    }
}

impl Test for ProblemService {
    fn test_object() -> Pin<Box<dyn Future<Output = Self> + Send>> {
        Box::pin(async move {
            let repo = ProblemRepository::test_object().await;
            Self { repo }
        })
    }
}

#[cfg(test)]
mod tests {
    use contracts::SolveRequest;

    type SqlxResult<T> = sqlx::Result<T>;

    use super::*;
    use rand::Rng;

    struct TestProblem {
        id: Option<i64>,
        data: String,
        answer: i64,
    }

    impl TestProblem {
        fn id(&self) -> SqlxResult<i64> {
            match self.id {
                Some(id) => Ok(id),
                None => Err(sqlx::Error::InvalidArgument("None".to_string())),
            }
        }

        fn set_id(&mut self, id: i64) {
            self.id = Some(id);
        }

        fn answer(&self) -> i64 {
            self.answer
        }

        fn data(&self) -> String {
            self.data.clone()
        }
    }
    impl Problem for TestProblem {
        type Data = String;
        fn create() -> String {
            let mut rng = rand::rng();
            let answer = rng.random_range(0..10);
            answer.to_string()
        }
        fn into_request(data: String) -> SolveRequest {
            SolveRequest::TestProblem { data }
        }
    }
    #[tokio::test]
    async fn insert_shouldwork() {
        let service = ProblemService::test_object().await;
        let problem1 = service.get::<TestProblem>().await.unwrap();
        let problem2 = service.get::<TestProblem>().await.unwrap();
        let problem3 = service.get::<TestProblem>().await.unwrap();
        assert_eq!(problem1.id, 1);
        assert_eq!(problem2.id, 2);
        assert_eq!(problem3.id, 3);

        let validation1 = service
            .validate(problem1.id, problem1.data.parse().unwrap())
            .await
            .unwrap();
        let validation2 = service
            .validate(problem1.id, problem2.data.parse().unwrap())
            .await
            .unwrap();
        let validation3 = service
            .validate(problem3.id, problem3.data.parse().unwrap())
            .await
            .unwrap();

        assert!(validation1);
        assert!(validation2);
        assert!(validation3);
    }
}
