mod fake {
    #[fnmock::fakeable]
    async fn async_function(value: String) -> String {
        format!("Real {}", value)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn test_async_function() {
            let result = async_function("Test".to_string()).await;
            assert_eq!(result, "Real Test");
        }

        #[tokio::test]
        async fn test_async_function_fake() {
            async_function_fake().setup(|value| format!("Fake {}", value));

            let result = async_function("Test".to_string()).await;
            assert_eq!(result, "Fake Test");
        }
    }
}

mod spy {
    #[fnmock::spyable]
    async fn async_function(value: String) -> String {
        format!("Real {}", value)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn test_async_function_spy() {
            let spy = async_function_spy();
            spy.expect(fnmock::predicate::eq("Test".to_string()));

            let result = async_function("Test".to_string()).await;
            assert_eq!(result, "Real Test");

            spy.assert();
        }
    }
}

mod mock {
    #[fnmock::mockable]
    async fn async_function(value: String) -> String {
        format!("Real {}", value)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn test_async_function() {
            let mock = async_function_mock();
            mock.setup(|value| format!("Fake {}", value));
            mock.expect(fnmock::predicate::eq("Test".to_string()))
                .once();

            let result = async_function("Test".to_string()).await;

            assert_eq!(result, "Fake Test");
            mock.assert();
        }
    }
}
