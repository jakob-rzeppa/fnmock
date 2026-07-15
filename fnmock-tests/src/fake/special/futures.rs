use std::{ future::Future, pin::Pin };

#[fnmock::fakeable]
fn futures(value: i32) -> Pin<Box<dyn Future<Output = String> + Send>> {
    Box::pin(async move { format!("Real {}", value) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_futures() {
        let result = futures(1).await;
        assert_eq!(result, "Real 1");
    }

    #[tokio::test]
    async fn test_futures_fake() {
        futures_fake().setup(|value| Box::pin(async move { format!("Fake {}", value) }));

        let result = futures(1).await;
        assert_eq!(result, "Fake 1");
    }
}
