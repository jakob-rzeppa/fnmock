//! A pub(super) function should not be accessible from a sibling module,
//! even with the fake accessor.

pub mod definitions {
    #[fnmock::fakeable]
    pub(super) fn pub_super_fn(a: String) -> String {
        format!("Real {}", a)
    }
}

mod sibling {
    fn try_access() {
        // Attempting to use the fake accessor for a pub(super) function should fail
        // because pub(super) is only accessible from the parent module
        super::definitions::pub_super_fn_fake().setup(|a| format!("Fake {}", a));
    }
}

fn main() {}
