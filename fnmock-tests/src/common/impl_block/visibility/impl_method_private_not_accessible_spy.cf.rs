//! A private impl method should not be accessible from a sibling module,
//! even with the spy accessor.

mod definitions {
    pub struct MyStruct;

    #[fnmock::spyable]
    impl MyStruct {
        fn private_method(&self, a: String) -> String {
            format!("Real {}", a)
        }
    }
}

mod sibling {
    use super::definitions::MyStruct;

    fn try_access() {
        // Attempting to use the spy accessor for a private impl method should fail
        MyStruct::private_method_spy().expect_once();
    }
}

fn main() {}
