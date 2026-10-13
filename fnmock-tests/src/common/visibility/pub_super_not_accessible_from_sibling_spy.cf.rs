//! A pub(super) function should not be accessible from outside its parent
//! module, even with the spy accessor.

mod parent {
    pub mod definitions {
        #[fnmock::spyable]
        pub(super) fn pub_super_fn(a: String) -> String {
            format!("Real {}", a)
        }
    }
}

mod sibling {
    fn try_access() {
        // Attempting to use the spy accessor for a pub(super) function should fail
        // because pub(super) is only accessible from the parent module and its descendants
        crate::parent::definitions::pub_super_fn_spy().expect_once();
    }
}

fn main() {}
