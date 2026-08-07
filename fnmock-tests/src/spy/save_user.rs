#[fnmock::spyable]
pub fn save_user(id: String) {
    let _ = id;
}
