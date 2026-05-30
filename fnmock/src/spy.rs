#[derive(Debug, Clone, PartialEq)]
pub struct IgnoredParam;

pub struct Spy<Args: Clone + PartialEq + 'static> {
    name: &'static str,
    calls: Vec<Args>,
}

impl<Args: Clone + PartialEq + 'static> Spy<Args> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            calls: Vec::new(),
        }
    }

    pub fn save(&mut self, args: Args) {
        self.calls.push(args);
    }

    pub fn clear(&mut self) {
        self.calls.clear();
    }

    pub fn assert_times(&self, times: usize) {
        assert_eq!(
            self.calls.len(),
            times,
            "Expected '{}' to be called {} times, but it was called {} times.",
            self.name,
            times,
            self.calls.len()
        );
    }

    pub fn assert_any(&self, args: fn(Args) -> bool) {
        assert!(self.calls.iter().any(|call_args| args(call_args.clone())));
    }

    pub fn assert_any_with(&self, args: Args) {
        assert!(self.calls.iter().any(|call_args| *call_args == args));
    }

    pub fn assert_nth(&self, n: usize, args: fn(Args) -> bool) {
        assert!(self.calls.get(n).map_or(false, |call_args| args(call_args.clone())));
    }

    pub fn assert_nth_with(&self, n: usize, args: Args) {
        assert!(self.calls.get(n).map_or(false, |call_args| *call_args == args));
    }
}
