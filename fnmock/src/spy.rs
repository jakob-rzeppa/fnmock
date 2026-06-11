#[derive(Debug, Clone, PartialEq)]
pub struct IgnoredParam;

pub struct Spy<Args: Clone + PartialEq> {
    name: &'static str,
    calls: Option<Vec<Args>>,
}

impl<Args: Clone + PartialEq> Spy<Args> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            calls: None,
        }
    }

    /// Sets up the spy to start recording calls. If the spy is already set up, this will panic.
    ///
    /// If you want to reset the spy, call clear() instead of setup() a second time.
    ///
    /// This method is separate from the constructor to allow for more flexible test setup.
    /// You can choose to set up the spy at the beginning of your tests, or only in specific test sections where you want to record calls.
    pub fn setup(&mut self) {
        if let Some(_) = &self.calls {
            panic!("{} spy already initialized. If you want to reset it, call clear().", self.name);
        }

        self.calls = Some(Vec::new());
    }

    /// Tears down the spy, clearing all recorded calls and preventing further calls from being recorded until setup() is called again.
    ///
    /// This method is separate from clear() to allow for more flexible test teardown.
    /// After tearing down the spy, it will not record any calls until setup() is called again,
    /// and any attempt to assert on calls will panic until the spy is set up again.
    pub fn teardown(&mut self) {
        self.calls = None;
    }

    /// Saves the arguments of a call to the spy. If the spy is not set up, this will not do anything.
    pub fn save(&mut self, args: Args) {
        if let Some(calls) = &mut self.calls {
            calls.push(args);
        }
    }

    /// Clears all recorded calls from the spy. If the spy is not set up, this will panic.
    pub fn clear(&mut self) {
        if let Some(calls) = &mut self.calls {
            calls.clear();
        } else {
            panic!("clear can't be executed on {}. Spy not initialized.", self.name)
        }
    }

    /// Returns true if the spy was called the specified number of times. If the spy is not set up, this will panic.
    pub fn called_times(&self, times: usize) -> bool {
        if let Some(calls) = &self.calls {
            calls.len() == times
        } else {
            panic!("called_times can't be executed on {}. Spy not initialized.", self.name)
        }
    }

    /// Returns true if any call to the spy matches the provided arguments according to the provided matching function.
    /// If the spy is not set up, this will panic.
    pub fn any_call_matches(&self, args: fn(Args) -> bool) -> bool {
        if let Some(calls) = &self.calls {
            calls.iter().any(|call_args| args(call_args.clone()))
        } else {
            panic!("any_call_matches can't be executed on {}. Spy not initialized.", self.name)
        }
    }

    /// Returns true if any call to the spy equals the provided arguments.
    /// If the spy is not set up, this will panic.
    pub fn any_call_equals(&self, args: Args) -> bool {
        if let Some(calls) = &self.calls {
            calls.iter().any(|call_args| *call_args == args)
        } else {
            panic!("any_call_equals can't be executed on {}. Spy not initialized.", self.name)
        }
    }

    /// Returns true if the nth call to the spy matches the provided arguments according to the provided matching function.
    /// If the spy is not set up, this will panic.
    pub fn nth_call_matches(&self, n: usize, args: fn(Args) -> bool) -> bool {
        if let Some(calls) = &self.calls {
            calls.get(n).map_or(false, |call_args| args(call_args.clone()))
        } else {
            panic!("nth_call_matches can't be executed on {}. Spy not initialized.", self.name)
        }
    }

    /// Returns true if the nth call to the spy equals the provided arguments.
    /// If the spy is not set up, this will panic.
    pub fn nth_call_equals(&self, n: usize, args: Args) -> bool {
        if let Some(calls) = &self.calls {
            calls.get(n).map_or(false, |call_args| *call_args == args)
        } else {
            panic!("nth_call_equals can't be executed on {}. Spy not initialized.", self.name)
        }
    }
}
