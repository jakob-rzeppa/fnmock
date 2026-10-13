//! A pub(super) function should not be accessible from outside its parent
//! module, even with the mock accessor.

mod parent {
    pub mod definitions {
        #[fnmock::mockable]
        pub(super) fn pub_super_fn(a: String) -> String {
            format!("Real {}", a)
        }
    }
}

mod sibling {
    fn try_access() {
        // Attempting to use the mock accessor for a pub(super) function should fail
        // because pub(super) is only accessible from the parent module and its descendants
        crate::parent::definitions::pub_super_fn_mock().setup(|a| format!("Fake {}", a));
    }
}

fn main() {}
