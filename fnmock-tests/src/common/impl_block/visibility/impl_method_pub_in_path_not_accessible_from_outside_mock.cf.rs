//! A pub(in path) impl method should not be accessible outside its specified path,
//! even with the mock accessor.

mod outer {
    pub mod inner {
        pub struct MyStruct;

        #[fnmock::mockable]
        impl MyStruct {
            pub(in crate::outer) fn path_restricted_method(&self, a: String) -> String {
                format!("Real {}", a)
            }
        }
    }
}

mod sibling {
    use crate::outer::inner::MyStruct;

    fn try_access() {
        // Attempting to access pub(in outer::inner) method from outside that path should fail
        MyStruct::path_restricted_method_mock().setup(|_, a| format!("Fake {}", a));
    }
}

fn main() {}
