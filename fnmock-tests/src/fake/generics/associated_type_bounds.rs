//! This test checks associated type bounds in generic functions.
//!
//! Associated type bounds are not enforced for the setup closure, only for the function itself.
//! You can setup a fake for a type that does not satisfy the associated type bounds, but calling the function with a type that does not satisfy the bounds will not be possible.
//!
//! Here you need to be extra careful, with the type the setup is called for, since you need to enforce the bound yourself,
//! otherwise you will have a unnecessary setup that can never be called.

use std::fmt::Display;

#[fnmock::fakeable]
fn associated_type_bounds<I>(value: I) -> Vec<String>
where
    I: Iterator + 'static,
    I::Item: Display,
{
    value.map(|item| item.to_string()).collect()
}

#[test]
fn test_associated_type_bounds() {
    let result = associated_type_bounds(vec![1i32, 2i32, 3i32].into_iter());
    assert_eq!(
        result,
        vec!["1".to_string(), "2".to_string(), "3".to_string()]
    );
}

#[test]
fn test_associated_type_bounds_fake() {
    associated_type_bounds_fake::<std::vec::IntoIter<i32>>()
        .setup(|_value| vec!["Fake".to_string()]);

    let result = associated_type_bounds(vec![1i32, 2i32, 3i32].into_iter());
    assert_eq!(result, vec!["Fake".to_string()]);
}
