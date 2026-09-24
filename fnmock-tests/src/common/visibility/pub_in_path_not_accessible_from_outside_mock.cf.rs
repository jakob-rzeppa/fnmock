//! A pub(in path) function should not be accessible outside its specified path,
//! even with the mock accessor.

mod outer {
    pub mod inner {
        #[fnmock::mockable]
        pub(in crate::outer) fn path_restricted_fn(a: String) -> String {
            format!("Real {}", a)
        }
    }
}

mod sibling {
    fn try_access() {
        // Attempting to access pub(in crate::outer) function from a different module should fail
        crate::outer::inner::path_restricted_fn_mock().setup(|a| format!("Fake {}", a));
    }
}

fn main() {}
