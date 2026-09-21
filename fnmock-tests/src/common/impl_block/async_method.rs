mod fake {
    struct AsyncMethod;

    #[fnmock::fakeable]
    impl AsyncMethod {
        async fn get_user(&self, user_id: u32) -> Option<String> {
            Some(format!("User{}", user_id))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn test_get_user() {
            let s = AsyncMethod;
            let result = s.get_user(1).await;
            assert_eq!(result, Some("User1".into()));
        }

        #[tokio::test]
        async fn test_get_user_mock() {
            AsyncMethod::get_user_fake().setup(|_, id| Some(format!("Fake{}", id)));

            let s = AsyncMethod;
            let result = s.get_user(1).await;
            assert_eq!(result, Some("Fake1".into()));
        }
    }
}

mod spy {
    struct AsyncMethod;

    #[fnmock::spyable]
    impl AsyncMethod {
        async fn get_user(&self, user_id: u32) -> Option<String> {
            Some(format!("User{}", user_id))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn test_get_user_spy() {
            let spy = AsyncMethod::get_user_spy();
            spy.expect_once();

            let s = AsyncMethod;
            let result = s.get_user(1).await;

            assert_eq!(result, Some("User1".into()));
            spy.assert();
        }
    }
}
