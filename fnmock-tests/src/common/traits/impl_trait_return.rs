//! Only works with `#[fnmock::spyable]` because of the way `impl Trait` works in Rust.
//! The return type is not a concrete type, so it cannot be mocked.
//! For spies, the return type is not relevant, so it can be used with `impl Trait`.

mod spy {
    #[fnmock::spyable]
    fn returns_impl_trait(value: i32) -> impl std::fmt::Display {
        value
    }
}

// No `mod mock` here: a mock is the intersection of fake and spy, and fake
// rejects `impl Trait` in return position (its closure bound has to name the
// return type), see docs/LIMITATIONS.md#return-types and
// unsupported_impl_trait_return_mock.cf.rs.
