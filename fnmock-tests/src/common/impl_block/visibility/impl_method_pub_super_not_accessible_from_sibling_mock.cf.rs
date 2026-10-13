//! A pub(super) impl method should not be accessible from outside its parent
//! module, only from the parent module and its descendants.

mod parent {
    pub mod definitions {
        pub struct MyStruct;

        #[fnmock::mockable]
        impl MyStruct {
            pub(super) fn parent_visible_method(&self, a: String) -> String {
                format!("Real {}", a)
            }
        }
    }
}

mod sibling {
    use crate::parent::definitions::MyStruct;

    fn try_access() {
        // Attempting to use the mock accessor for a pub(super) impl method
        // from outside the parent module should fail
        MyStruct::parent_visible_method_mock().setup(|_, a| format!("Fake {}", a));
    }
}

fn main() {}
