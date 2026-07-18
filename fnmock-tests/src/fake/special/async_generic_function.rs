use std::fmt::Display;

#[fnmock::fakeable]
async fn async_generic_function<T: Display + 'static>(value: T) -> String {
    format!("Real {}", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_generic_function() {
        let result = async_generic_function(1).await;
        assert_eq!(result, "Real 1");
    }

    #[tokio::test]
    async fn test_async_generic_function_fake() {
        async_generic_function_fake::<i32>().setup(|value| format!("Fake {}", value));

        let result = async_generic_function(1).await;
        assert_eq!(result, "Fake 1");
    }
}
