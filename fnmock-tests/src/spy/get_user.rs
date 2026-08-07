#[fnmock::spyable]
pub fn get_user(mut id: String, uuid: &str) -> String {
    id.push_str(uuid);
    id
}
