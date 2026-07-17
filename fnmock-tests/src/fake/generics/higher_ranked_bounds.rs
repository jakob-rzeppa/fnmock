use std::fmt::Debug;

#[fnmock::fakeable]
fn higher_ranked_bounds_closure<F>(value: F) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str + 'static,
{
    value("Real").to_string()
}

#[test]
fn test_higher_ranked_bounds_closure() {
    let result = higher_ranked_bounds_closure::<for<'a> fn(&'a str) -> &'a str>(|value| value);
    assert_eq!(result, "Real");
}

#[test]
fn test_higher_ranked_bounds_closure_fake() {
    higher_ranked_bounds_closure_fake::<for<'a> fn(&'a str) -> &'a str>()
        .setup(|_value| "Fake".to_string());

    let result = higher_ranked_bounds_closure::<for<'a> fn(&'a str) -> &'a str>(|value| value);
    assert_eq!(result, "Fake");
}

trait HigherRankedBoundsTrait<T>: Debug {}

#[derive(Debug)]
struct MyStruct {
    data: String,
}

impl HigherRankedBoundsTrait<&str> for MyStruct {}

#[fnmock::fakeable]
fn higher_ranked_bounds_trait<I>(value: I) -> String
where
    I: for<'a> HigherRankedBoundsTrait<&'a str> + 'static,
{
    format!("{:?}", value)
}

#[test]
fn test_higher_ranked_bounds_trait() {
    let result = higher_ranked_bounds_trait::<MyStruct>(MyStruct {
        data: "Real".to_string(),
    });
    assert_eq!(result, "MyStruct { data: \"Real\" }");
}

#[test]
fn test_higher_ranked_bounds_trait_fake() {
    higher_ranked_bounds_trait_fake::<MyStruct>().setup(|_value| "Fake".to_string());

    let result = higher_ranked_bounds_trait::<MyStruct>(MyStruct {
        data: "Fake".to_string(),
    });
    assert_eq!(result, "Fake");
}
