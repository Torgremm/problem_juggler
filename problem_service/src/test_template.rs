use std::pin::Pin;

pub trait Test {
    fn test_object() -> Pin<Box<dyn Future<Output = Self> + Send>>;
}
