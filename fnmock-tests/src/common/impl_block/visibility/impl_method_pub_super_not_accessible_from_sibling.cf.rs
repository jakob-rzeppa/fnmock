//! A pub(super) impl method should not be accessible from a sibling module,
//! only from the parent module.

mod definitions {
    pub struct MyStruct;

    #[fnmock::fakeable]
    impl MyStruct {
        pub(super) fn parent_visible_method(&self, a: String) -> String {
            format!("Real {}", a)
        }
    }
}

mod sibling {
    use super::definitions::MyStruct;

    fn try_access() {
        // Attempting to use the fake accessor for a pub(super) impl method
        // from a sibling module should fail
        MyStruct::parent_visible_method_fake().setup(|_, a| format!("Fake {}", a));
    }
}

fn main() {}
