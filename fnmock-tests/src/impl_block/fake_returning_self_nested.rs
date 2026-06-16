use std::{ collections::HashMap, sync::{ Arc, Mutex } };

#[derive(Debug, PartialEq)]
struct User {
    name: String,
}

#[fnmock::fakeable]
impl User {
    fn new(name: &str) -> Result<Box<HashMap<String, (Vec<Self>, String)>>, String> {
        let mut res = Box::new(HashMap::new());

        res.insert(name.to_string(), (
            vec![User {
                name: name.to_string(),
            }],
            name.to_string(),
        ));

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use fnmock::fake;

    use super::*;

    #[test]
    fn test_new() {
        let res = User::new("Alice");

        assert_eq!(
            res.unwrap().get("Alice").unwrap(),
            &(
                vec![User {
                    name: "Alice".to_string(),
                }],
                "Alice".to_string(),
            )
        );
    }

    #[test]
    fn test_fake_new() {
        fake!(User, new).setup(|name| {
            let name = format!("Fake{}", name);

            let mut res = Box::new(HashMap::new());

            res.insert(name.to_string(), (
                vec![User {
                    name: name.to_string(),
                }],
                name.to_string(),
            ));

            Ok(res)
        });

        let res = User::new("Bob");

        assert_eq!(
            res.unwrap().get("FakeBob").unwrap(),
            &(
                vec![User {
                    name: "FakeBob".to_string(),
                }],
                "FakeBob".to_string(),
            )
        );
    }
}
