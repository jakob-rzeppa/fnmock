//! A private function should not be accessible from a sibling module,
//! even with the mock accessor.

pub mod definitions {
    #[fnmock::mockable]
    fn private_fn(a: String) -> String {
        format!("Real {}", a)
    }
}

mod sibling {
    fn try_access() {
        // Attempting to use the mock accessor for a private function should fail
        super::definitions::private_fn_mock().setup(|a| format!("Fake {}", a));
    }
}

fn main() {}
