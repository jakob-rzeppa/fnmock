//! A private function should not be accessible from a sibling module,
//! even with the fake accessor.

pub mod definitions {
    #[fnmock::fakeable]
    fn private_fn(a: String) -> String {
        format!("Real {}", a)
    }
}

mod sibling {
    fn try_access() {
        // Attempting to use the fake accessor for a private function should fail
        super::definitions::private_fn_fake().setup(|a| format!("Fake {}", a));
    }
}

fn main() {}
