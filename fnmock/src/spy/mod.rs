use std::{ fmt::Debug, ops::RangeBounds };

use crate::spy::{
    call_expectations::CallExpectations,
    range::CallRange,
    times_expectation::TimesExpectation,
};

mod times_expectation;
mod call_expectations;
mod range;

pub struct Spy<Args: Debug> {
    name: &'static str,

    /// Expectations about the arguments of calls to the function.
    call_expectations: Option<CallExpectations<Args>>,

    /// Expectations about the total number of calls to the function.
    times_expectation: Option<TimesExpectation>,
}

impl<Args: Debug> Spy<Args> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            call_expectations: None,
            times_expectation: None,
        }
    }

    /// Sets up the spy to set expectations and record calls. If the spy is already set up, this will panic.
    ///
    /// Note that you can call setup() multiple times on the same spy, but you need to call verify() in between to be able to set up the spy again.
    /// This is because verify() will check and clear the expectations, so you can set up the spy again to set new expectations.
    ///
    /// You can choose to set up the spy at the beginning of your tests, or only in specific test sections where you want to record calls.
    /// Also, by using setup() the spy will only check expectations in tests where it is explicitly set up,
    /// so you can have multiple tests for the same function, and only set up the spy in the tests where you want to verify expectations.
    pub fn setup(&mut self) {
        if let Some(_) = &self.call_expectations {
            panic!(
                "{} spy already initialized. You need to call verify(), to be able to setup the spy again.",
                self.name
            );
        }

        self.call_expectations = Some(CallExpectations::new());
    }

    /// Check if a expectation is met for the given args
    pub fn record(&mut self, args: Args) -> Args {
        if let Some(call_expectations) = &mut self.call_expectations {
            match call_expectations.record_call(&args) {
                Ok(()) => (),
                Err(e) => panic!("Expectation for {} spy were not met: {}", self.name, e),
            }

            if let Some(times_expectation) = &mut self.times_expectation {
                times_expectation.increment_times_called();
            }
        }
        args
    }

    pub fn verify(&mut self) {
        if let Some(call_expectations) = &self.call_expectations {
            match call_expectations.is_met() {
                Ok(()) => (),
                Err(e) => panic!("Expectations for {} spy were not met: {}", self.name, e),
            }
        } else {
            panic!(
                "{} spy not initialized. You need to call setup() before verifying expectations.",
                self.name
            );
        }

        if let Some(times_expectation) = &self.times_expectation {
            match times_expectation.is_met() {
                Ok(()) => (),
                Err(e) => panic!("Expectations for {} spy were not met: {}", self.name, e),
            }
        }

        // Clear expectations and recorded calls, so that the spy can be set up again if needed.
        self.call_expectations = None;
        self.times_expectation = None;
    }

    pub fn expect_call_range<R: RangeBounds<usize>>(
        &mut self,
        range: R,
        expectation: fn(&Args) -> bool,
        expectation_display: &'static str
    ) {
        self.call_expectations
            .as_mut()
            .expect("Spy not initialized. You need to call setup() before setting expectations.")
            .add_expectation(expectation, CallRange::from_range(range), expectation_display);
    }

    /// Expects a call with the given arguments to be made at least once.
    pub fn expect_call(
        &mut self,
        expectation: fn(&Args) -> bool,
        expectation_display: &'static str
    ) {
        self.expect_call_range(1.., expectation, expectation_display)
    }

    /// Expects a call with the given arguments to be made exactly once.
    pub fn expect_call_once(
        &mut self,
        expectation: fn(&Args) -> bool,
        expectation_display: &'static str
    ) {
        self.expect_call_range(1..=1, expectation, expectation_display)
    }

    /// Expects a call with the given arguments to be made a specific number of times.
    pub fn expect_call_times(
        &mut self,
        times: usize,
        expectation: fn(&Args) -> bool,
        expectation_display: &'static str
    ) {
        self.expect_call_range(times..=times, expectation, expectation_display)
    }

    pub fn in_sequence(&mut self) {
        self.call_expectations
            .as_mut()
            .expect("Spy not initialized. You need to call setup() before setting expectations.")
            .set_in_sequence();
    }

    pub fn expect_range<R: RangeBounds<usize>>(&mut self, range: R) {
        if self.call_expectations.is_none() {
            panic!("Spy not initialized. You need to call setup() before setting expectations.");
        }

        if self.times_expectation.is_some() {
            panic!(
                "Times expectation already set. You can only set one times expectation per spy."
            );
        }

        self.times_expectation = Some(TimesExpectation::new(CallRange::from_range(range)));
    }

    pub fn expect_times(&mut self, n: usize) {
        self.expect_range(n..=n)
    }

    pub fn expect_never(&mut self) {
        self.expect_range(0..=0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_one(v: &i32) -> bool {
        *v == 1
    }

    fn is_two(v: &i32) -> bool {
        *v == 2
    }

    //
    // Lifecycle
    //

    #[test]
    fn setup_then_verify_without_expectations_succeeds() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.verify();
    }

    #[test]
    fn verify_clears_state_allowing_setup_again() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.verify();
        spy.setup();
        spy.verify();
    }

    #[test]
    #[should_panic(expected = "spy not initialized")]
    fn verify_without_setup_panics() {
        let mut spy = Spy::<i32>::new("test");
        spy.verify();
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn setup_twice_without_verify_panics() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.setup();
    }

    #[test]
    fn record_calls_succeeds_after_setup() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.record(1);
        spy.record(2);
        spy.verify();
    }

    #[test]
    fn record_calls_without_setup_succeeds_and_does_not_panic() {
        let mut spy = Spy::<i32>::new("test");
        spy.record(1);
        spy.record(2);
    }

    //
    // Expectation setup errors
    //

    #[test]
    #[should_panic(expected = "Spy not initialized")]
    fn expect_call_before_setup_panics() {
        let mut spy = Spy::<i32>::new("test");
        spy.expect_call(is_one, "1");
    }

    #[test]
    #[should_panic(expected = "Spy not initialized")]
    fn expect_call_once_before_setup_panics() {
        let mut spy = Spy::<i32>::new("test");
        spy.expect_call_once(is_one, "1");
    }

    #[test]
    #[should_panic(expected = "Spy not initialized")]
    fn expect_call_times_before_setup_panics() {
        let mut spy = Spy::<i32>::new("test");
        spy.expect_call_times(2, is_one, "1");
    }

    #[test]
    #[should_panic(expected = "Spy not initialized")]
    fn in_sequence_before_setup_panics() {
        let mut spy = Spy::<i32>::new("test");
        spy.in_sequence();
    }

    #[test]
    #[should_panic(expected = "Spy not initialized")]
    fn expect_range_before_setup_panics() {
        let mut spy = Spy::<i32>::new("test");
        spy.expect_range(1..=2);
    }

    //
    // expect_call (>=1)
    //

    #[test]
    fn expect_call_succeeds_when_called_once() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call(is_one, "1");
        spy.record(1);
        spy.verify();
    }

    #[test]
    fn expect_call_succeeds_when_called_multiple_times() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call(is_one, "1");
        spy.record(1);
        spy.record(1);
        spy.record(1);
        spy.verify();
    }

    #[test]
    #[should_panic]
    fn expect_call_fails_when_never_called() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call(is_one, "1");
        spy.verify();
    }

    //
    // expect_call_once
    //

    #[test]
    fn expect_call_once_succeeds() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call_once(is_one, "1");
        spy.record(1);
        spy.verify();
    }

    #[test]
    #[should_panic]
    fn expect_call_once_fails_when_missing() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call_once(is_one, "1");
        spy.verify();
    }

    #[test]
    #[should_panic]
    fn expect_call_once_fails_when_called_twice() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call_once(is_one, "1");
        spy.record(1);
        spy.record(1);
        spy.verify();
    }

    //
    // expect_call_times
    //

    #[test]
    fn expect_call_times_exact_match_succeeds() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call_times(2, is_one, "1");
        spy.record(1);
        spy.record(1);
        spy.verify();
    }

    #[test]
    #[should_panic]
    fn expect_call_times_too_few_fails() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call_times(2, is_one, "1");
        spy.record(1);
        spy.verify();
    }

    #[test]
    #[should_panic]
    fn expect_call_times_too_many_fails() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call_times(2, is_one, "1");
        spy.record(1);
        spy.record(1);
        spy.record(1);
        spy.verify();
    }

    //
    // Multiple call expectations
    //

    #[test]
    fn multiple_expectations_all_met_succeed() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call(is_one, "1");
        spy.expect_call(is_two, "2");
        spy.record(1);
        spy.record(2);
        spy.verify();
    }

    #[test]
    #[should_panic]
    fn multiple_expectations_fail_when_one_missing() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call(is_one, "1");
        spy.expect_call(is_two, "2");
        spy.record(1);
        spy.verify();
    }

    //
    // Times expectations
    //

    #[test]
    fn expect_times_exact_match_succeeds() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_times(2);
        spy.record(1);
        spy.record(2);
        spy.verify();
    }

    #[test]
    #[should_panic]
    fn expect_times_too_few_fails() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_times(2);
        spy.record(1);
        spy.verify();
    }

    #[test]
    #[should_panic]
    fn expect_times_too_many_fails() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_times(2);
        spy.record(1);
        spy.record(2);
        spy.record(1);
        spy.verify();
    }

    //
    // expect_never
    //

    #[test]
    fn expect_never_succeeds_when_no_calls_are_made() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_never();
        spy.verify();
    }

    #[test]
    #[should_panic]
    fn expect_never_fails_when_called() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_never();
        spy.record(1);
        spy.verify();
    }

    //
    // Times expectation configuration
    //

    #[test]
    #[should_panic(expected = "Times expectation already set")]
    fn multiple_times_expectations_panics() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_times(1);
        spy.expect_never();
    }

    //
    // Combined call + times expectations
    //

    #[test]
    fn call_and_times_expectations_both_met() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call_once(is_one, "1");
        spy.expect_times(1);
        spy.record(1);
        spy.verify();
    }

    #[test]
    #[should_panic]
    fn call_expectation_met_but_times_fails() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call_once(is_one, "1");
        spy.expect_times(2);
        spy.record(1);
        spy.verify();
    }

    #[test]
    #[should_panic]
    fn times_met_but_call_expectation_fails() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call_once(is_two, "2");
        spy.expect_times(1);
        spy.record(1);
        spy.verify();
    }

    //
    // in_sequence
    //

    #[test]
    fn in_sequence_succeeds_when_calls_made_in_order() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call(is_one, "1");
        spy.expect_call(is_two, "2");
        spy.in_sequence();
        spy.record(1);
        spy.record(2);
        spy.verify();
    }

    #[test]
    #[should_panic]
    fn in_sequence_fails_when_calls_made_out_of_order() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call(is_one, "1");
        spy.expect_call(is_two, "2");
        spy.in_sequence();
        spy.record(2);
        spy.record(1);
        spy.verify();
    }

    #[test]
    #[should_panic]
    fn in_sequence_fails_when_calls_made_out_of_order_and_one_missing() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call(is_one, "1");
        spy.expect_call(is_two, "2");
        spy.in_sequence();
        spy.record(2);
        spy.verify();
    }

    #[test]
    fn in_sequence_expect_call_allows_multiple_calls_as_long_as_order_is_maintained() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call(is_one, "1");
        spy.expect_call(is_two, "2");
        spy.in_sequence();
        spy.record(1);
        spy.record(1);
        spy.record(2);
        spy.record(2);
        spy.verify();
    }

    #[test]
    #[should_panic]
    fn in_sequence_expect_call_once_fails_when_called_more_than_once() {
        let mut spy = Spy::<i32>::new("test");
        spy.setup();
        spy.expect_call_once(is_one, "1");
        spy.expect_call_once(is_two, "2");
        spy.in_sequence();
        spy.record(1);
        spy.record(2);
        spy.record(2);
        spy.verify();
    }
}
