use std::fmt::Display;

struct GenericMethodAsync<T> {
    value: T,
}

#[fnmock::fakeable]
impl<T: Display + 'static> GenericMethodAsync<T> {
    async fn combine<U: Display + 'static>(&self, other: U) -> String {
        format!("{} {}", self.value, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generic_method_async() {
        let s = GenericMethodAsync {
            value: "Test".to_string(),
        };
        let result = s.combine(1).await;
        assert_eq!(result, "Test 1");
    }

    #[tokio::test]
    async fn test_generic_method_async_fake() {
        GenericMethodAsync::<String>::combine_fake::<i32>()
            .setup(|_, other| format!("Fake {}", other));

        let s = GenericMethodAsync {
            value: "Test".to_string(),
        };
        let result = s.combine(1).await;
        assert_eq!(result, "Fake 1");
    }
}
