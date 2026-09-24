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

// No `mod mock` here: a mock is the intersection of fake and spy, and fake
// rejects `!` as a return type (there is no value for its closure to
// produce), see docs/LIMITATIONS.md#return-types and
// unsupported_never_return_type_mock.cf.rs.
