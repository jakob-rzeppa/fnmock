struct LifetimesAndGenerics;

#[fnmock::fakeable]
impl LifetimesAndGenerics {
    fn describe<'a, T: std::fmt::Display + 'static>(&self, prefix: &'a str, value: T) -> String {
        format!("{}: {}", prefix, value)
    }
}

#[test]
fn test_lifetimes_and_generics() {
    let s = LifetimesAndGenerics;
    assert_eq!(s.describe("Value", 42), "Value: 42");
}

#[test]
fn test_lifetimes_and_generics_fake() {
    LifetimesAndGenerics::describe_fake::<i32>().setup(|_, prefix, value| {
        format!("Fake {}: {}", prefix, value)
    });

    let s = LifetimesAndGenerics;
    assert_eq!(s.describe("Value", 42), "Fake Value: 42");
}
