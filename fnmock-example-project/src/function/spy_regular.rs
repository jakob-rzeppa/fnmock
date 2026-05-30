fn get_user(id: i32) -> String {
    #[cfg(test)]
    get_user_spy::save(id);

    format!("User {}", id)
}

#[cfg(test)]
pub(crate) mod get_user_spy {
    use fnmock::spy::Spy;

    thread_local! {
        static FAKE: std::cell::RefCell<Spy<i32>> = std::cell::RefCell::new(Spy::new("get_user"));
    }

    pub(crate) fn save(args: i32) {
        FAKE.with_borrow_mut(|spy| {
            spy.save(args);
        });
    }

    pub(crate) fn clear() {
        FAKE.with_borrow_mut(|spy| {
            spy.clear();
        })
    }

    pub(crate) fn assert_times(times: usize) {
        FAKE.with_borrow(|spy| {
            spy.assert_times(times);
        })
    }

    pub(crate) fn assert_any(args: fn(i32) -> bool) {
        FAKE.with_borrow(|spy| {
            spy.assert_any(args);
        })
    }

    pub(crate) fn assert_any_with(args: i32) {
        FAKE.with_borrow(|spy| {
            spy.assert_any_with(args);
        })
    }

    pub(crate) fn assert_nth(n: usize, args: fn(i32) -> bool) {
        FAKE.with_borrow(|spy| {
            spy.assert_nth(n, args);
        })
    }

    pub(crate) fn assert_nth_with(n: usize, args: i32) {
        FAKE.with_borrow(|spy| {
            spy.assert_nth_with(n, args);
        })
    }
}

fn handle_user(id: i32) -> String {
    let user = get_user(id);
    user
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_user_spy() {
        let result = handle_user(1);

        assert_eq!(result, "User 1");

        get_user_spy::assert_times(1);
        get_user_spy::assert_any(|id| id == 1);
        get_user_spy::assert_any_with(1);
        get_user_spy::assert_nth(0, |id| id == 1);
        get_user_spy::assert_nth_with(0, 1);

        get_user_spy::clear();

        let result = handle_user(2);

        assert_eq!(result, "User 2");

        get_user_spy::assert_times(1);
        get_user_spy::assert_any_with(2);
        get_user_spy::assert_nth_with(0, 2);
    }
}
