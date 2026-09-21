mod fake {
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
}

mod spy {
    use std::fmt::Display;

    #[fnmock::spyable]
    async fn async_generic_function<T: Display + 'static>(value: T) -> String {
        format!("Real {}", value)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn test_async_generic_function_spy() {
            let spy = async_generic_function_spy::<i32>();
            spy.expect(fnmock::predicate::eq(1));

            let result = async_generic_function(1).await;
            assert_eq!(result, "Real 1");

            spy.assert();
        }
    }
}
