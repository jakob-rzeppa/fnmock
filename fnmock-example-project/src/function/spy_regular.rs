fn get_user(id: i32) -> String {
    #[cfg(test)]
    get_user_spy::save(id);

    format!("User {}", id)
}

#[cfg(test)]
pub(crate) mod get_user_spy {
    use fnmock::spy::Spy;

    thread_local! {
        static GET_USER_SPY: std::cell::RefCell<Spy<i32>> = std::cell::RefCell::new(
            Spy::new("get_user")
        );
    }

    pub(crate) fn setup() {
        GET_USER_SPY.with_borrow_mut(|spy| {
            spy.setup();
        });
    }

    pub(crate) fn teardown() {
        GET_USER_SPY.with_borrow_mut(|spy| {
            spy.teardown();
        });
    }

    pub(crate) fn save(args: i32) {
        GET_USER_SPY.with_borrow_mut(|spy| {
            spy.save(args);
        });
    }

    pub(crate) fn clear() {
        GET_USER_SPY.with_borrow_mut(|spy| {
            spy.clear();
        })
    }

    pub(crate) fn called_times(times: usize) -> bool {
        GET_USER_SPY.with_borrow(|spy| { spy.called_times(times) })
    }

    pub(crate) fn assert_called_times(times: usize) {
        GET_USER_SPY.with_borrow(|spy| { assert!(spy.called_times(times)) })
    }

    pub(crate) fn any_call_matches(args: fn(i32) -> bool) -> bool {
        GET_USER_SPY.with_borrow(|spy| { spy.any_call_matches(args) })
    }

    pub(crate) fn assert_any_call_matches(args: fn(i32) -> bool) {
        GET_USER_SPY.with_borrow(|spy| { assert!(spy.any_call_matches(args)) })
    }

    pub(crate) fn any_call_equals(args: i32) -> bool {
        GET_USER_SPY.with_borrow(|spy| { spy.any_call_equals(args) })
    }

    pub(crate) fn assert_any_call_equals(args: i32) {
        GET_USER_SPY.with_borrow(|spy| { assert!(spy.any_call_equals(args)) })
    }

    pub(crate) fn nth_call_matches(n: usize, args: fn(i32) -> bool) -> bool {
        GET_USER_SPY.with_borrow(|spy| { spy.nth_call_matches(n, args) })
    }

    pub(crate) fn assert_nth_call_matches(n: usize, args: fn(i32) -> bool) {
        GET_USER_SPY.with_borrow(|spy| { assert!(spy.nth_call_matches(n, args)) })
    }

    pub(crate) fn nth_call_equals(n: usize, args: i32) -> bool {
        GET_USER_SPY.with_borrow(|spy| { spy.nth_call_equals(n, args) })
    }

    pub(crate) fn assert_nth_call_equals(n: usize, args: i32) {
        GET_USER_SPY.with_borrow(|spy| { assert!(spy.nth_call_equals(n, args)) })
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
        get_user_spy::setup();

        let result = handle_user(1);

        assert_eq!(result, "User 1");

        assert!(get_user_spy::called_times(1));
        get_user_spy::assert_called_times(1);
        get_user_spy::assert_any_call_matches(|id| id == 1);
        get_user_spy::assert_any_call_equals(1);
        get_user_spy::assert_nth_call_matches(0, |id| id == 1);
        get_user_spy::assert_nth_call_equals(0, 1);

        get_user_spy::clear();

        let result = handle_user(2);

        assert_eq!(result, "User 2");

        get_user_spy::assert_called_times(1);
        get_user_spy::assert_any_call_matches(|id| id == 2);
        get_user_spy::assert_nth_call_matches(0, |id| id == 2);
    }
}
