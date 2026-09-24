//! A private function should not be accessible from a sibling module,
//! even with the spy accessor.

pub mod definitions {
    #[fnmock::spyable]
    fn private_fn(a: String) -> String {
        format!("Real {}", a)
    }
}

mod sibling {
    fn try_access() {
        // Attempting to use the spy accessor for a private function should fail
        super::definitions::private_fn_spy().expect_once();
    }
}

fn main() {}
