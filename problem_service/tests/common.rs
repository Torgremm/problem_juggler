use problem_service::ProblemService;
use tokio::sync::OnceCell;

static TEST_SERVICE: OnceCell<ProblemService> = OnceCell::const_new();

pub async fn get_test_service() -> &'static ProblemService {
    TEST_SERVICE
        .get_or_init(|| async { problem_service::test_utils::test_service().await })
        .await
}
