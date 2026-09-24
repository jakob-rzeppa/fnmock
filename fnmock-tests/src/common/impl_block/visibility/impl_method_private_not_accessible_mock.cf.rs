//! A private impl method should not be accessible from a sibling module,
//! even with the mock accessor.

mod definitions {
    pub struct MyStruct;

    #[fnmock::mockable]
    impl MyStruct {
        fn private_method(&self, a: String) -> String {
            format!("Real {}", a)
        }
    }
}

mod sibling {
    use super::definitions::MyStruct;

    fn try_access() {
        // Attempting to use the mock accessor for a private impl method should fail
        MyStruct::private_method_mock().setup(|_, a| format!("Fake {}", a));
    }
}

fn main() {}
