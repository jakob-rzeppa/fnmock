struct Foo<'a>(&'a str);

#[fnmock::fakeable]
impl<'a> Foo<'a> {
    fn show<T: 'a + std::fmt::Display>(&self, value: T) -> String {
        format!("{}{value}", self.0)
    }
}

fn main() {}
