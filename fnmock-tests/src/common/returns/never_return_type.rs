//! Only works with `#[fnmock::spyable]` macro.

mod spy {
    #[fnmock::spyable]
    fn returns_never(flag: bool) -> ! {
        let _ = flag;
        panic!("This function never returns!");
    }

    // We can't use this in a test trivially, but a simple compilation test is enough,
    // since a problem with the never return type would cause a compilation error.
}
