//! A private impl method should not be accessible from a sibling module,
//! even with the fake accessor.

mod definitions {
    pub struct MyStruct;

    #[fnmock::fakeable]
    impl MyStruct {
        fn private_method(&self, a: String) -> String {
            format!("Real {}", a)
        }
    }
}

mod sibling {
    use super::definitions::MyStruct;

    fn try_access() {
        // Attempting to use the fake accessor for a private impl method should fail
        MyStruct::private_method_fake().setup(|_, a| format!("Fake {}", a));
    }
}

fn main() {}
