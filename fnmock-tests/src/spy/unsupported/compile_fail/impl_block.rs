//! `#[spyable]` does not support impl blocks yet — only free functions.

struct UserService;

#[fnmock::spyable]
impl UserService {
    fn get_user(&self, id: String) -> String {
        id
    }
}

fn main() {}
