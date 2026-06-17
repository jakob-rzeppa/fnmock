use std::{ collections::VecDeque, fmt::Debug, ops::Bound };

use crate::spy::range::CallRange;

pub struct CallExpectations<Args: Debug> {
    calls: VecDeque<CallExpectation<Args>>,
    in_sequence: bool,
}

impl<Args: Debug> CallExpectations<Args> {
    pub fn new() -> Self {
        Self {
            calls: VecDeque::new(),
            in_sequence: false,
        }
    }

    pub fn set_in_sequence(&mut self) {
        self.in_sequence = true;
    }

    pub fn add_expectation(
        &mut self,
        expectation: fn(&Args) -> bool,
        call_range: CallRange,
        expectation_display: &'static str
    ) {
        self.calls.push_back(CallExpectation {
            expectation,
            call_range,
            expectation_display,
            times_called: 0,
        });
    }

    pub fn record_call(&mut self, args: &Args) -> Result<(), String> {
        if self.in_sequence {
            self.record_call_in_sequence(args)?;
        } else {
            self.record_call_not_in_sequence(args);
        }
        Ok(())
    }

    fn record_call_not_in_sequence(&mut self, args: &Args) {
        for call in &mut self.calls {
            if (call.expectation)(args) {
                call.times_called += 1;
            }
        }
    }

    fn record_call_in_sequence(&mut self, args: &Args) -> Result<(), String> {
        if let Some(call) = self.calls.front_mut() {
            if (call.expectation)(args) {
                call.times_called += 1;
                if call.call_range.is_at_max(call.times_called) {
                    self.calls.pop_front();
                }
            } else {
                if
                    call.call_range.is_within(call.times_called) // If the current expectation is already satisfied, we can skip it and check the next one
                {
                    self.calls.pop_front();

                    // Recursively check the next expectation with the same call
                    // This allows us to skip over expectation bounds that have already been satisfied
                    self.record_call_in_sequence(args)?;
                } else {
                    return Err(
                        format!(
                            "{} is not valid for {:?}. Expected {}, but got {}",
                            call.expectation_display,
                            args,
                            call.call_range,
                            call.times_called
                        )
                    );
                }
            }
            Ok(())
        } else {
            Err(
                format!(
                    "Arguments ({:?}) are not expected. Every expectation has already been satisfied.",
                    args
                )
            )
        }
    }

    pub fn is_met(&self) -> Result<(), String> {
        if self.in_sequence { self.is_met_in_sequence() } else { self.is_met_not_in_sequence() }
    }

    fn is_met_not_in_sequence(&self) -> Result<(), String> {
        let mut errors = Vec::new();
        for call in &self.calls {
            if let Err(e) = call.is_met() {
                errors.push(e);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(format!("Not all expectations were satisfied: [{}]", errors.join(", ")))
        }
    }

    fn is_met_in_sequence(&self) -> Result<(), String> {
        if self.calls.is_empty() {
            Ok(())
        } else {
            let call = self.calls
                .front()
                .expect("If deque is not empty .front() must return a value.");

            if call.call_range.is_within(call.times_called) {
                Ok(())
            } else {
                Err(
                    format!(
                        "Not all expectations were satisfied. Next expectation {} has {} calls, expected {}.",
                        call.expectation_display,
                        call.times_called,
                        call.call_range
                    )
                )
            }
        }
    }
}

struct CallExpectation<Args> {
    expectation: fn(&Args) -> bool,
    expectation_display: &'static str,

    call_range: CallRange,
    times_called: usize,
}

impl<Args: Debug> CallExpectation<Args> {
    fn is_met(&self) -> Result<(), String> {
        if self.call_range.is_within(self.times_called) {
            Ok(())
        } else {
            Err(
                format!(
                    "Expectation {} not met. Expected {}, but got {}.",
                    self.expectation_display,
                    self.call_range,
                    self.times_called
                )
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_in_sequence_success() {
        let mut expectations = CallExpectations::new();
        expectations.add_expectation(
            |&x| x == 1,
            CallRange::new(Bound::Included(1), Bound::Included(2)),
            "x == 1"
        );
        expectations.add_expectation(
            |&x| x == 2,
            CallRange::new(Bound::Included(1), Bound::Included(2)),
            "x == 2"
        );

        expectations.record_call(&1).unwrap();
        expectations.record_call(&2).unwrap();
        expectations.record_call(&1).unwrap();
        expectations.record_call(&2).unwrap();

        assert_eq!(expectations.calls[0].times_called, 2);
        assert_eq!(expectations.calls[1].times_called, 2);

        assert!(expectations.is_met().is_ok());
    }

    #[test]
    fn test_not_in_sequence_failure() {
        let mut expectations = CallExpectations::new();
        expectations.add_expectation(
            |&x| x == 1,
            CallRange::new(Bound::Included(3), Bound::Included(3)),
            "x == 1"
        );
        expectations.add_expectation(
            |&x| x == 2,
            CallRange::new(Bound::Included(1), Bound::Included(2)),
            "x == 2"
        );

        expectations.record_call(&1).unwrap();
        expectations.record_call(&2).unwrap();
        expectations.record_call(&1).unwrap();
        expectations.record_call(&2).unwrap();

        assert_eq!(expectations.calls[0].times_called, 2); // This is the expectation that will fail, since it expected exactly 3 calls but only got 2
        assert_eq!(expectations.calls[1].times_called, 2);

        let result = expectations.is_met();
        assert!(result.is_err());
        let error_message = result.err().unwrap();
        assert_eq!(
            error_message,
            "Not all expectations were satisfied: [Expectation x == 1 not met. Expected exactly 3 calls, but got 2.]"
        );
    }

    #[test]
    fn test_not_in_sequence_multiple_failures() {
        let mut expectations = CallExpectations::new();
        expectations.add_expectation(
            |&x| x == 1,
            CallRange::new(Bound::Included(3), Bound::Included(3)),
            "x == 1"
        );
        expectations.add_expectation(
            |&x| x == 2,
            CallRange::new(Bound::Included(3), Bound::Included(3)),
            "x == 2"
        );

        expectations.record_call(&1).unwrap();
        expectations.record_call(&2).unwrap();
        expectations.record_call(&1).unwrap();
        expectations.record_call(&2).unwrap();

        assert_eq!(expectations.calls[0].times_called, 2);
        assert_eq!(expectations.calls[1].times_called, 2);

        let result = expectations.is_met();
        assert!(result.is_err());
        let error_message = result.err().unwrap();
        assert_eq!(
            error_message,
            "Not all expectations were satisfied: [Expectation x == 1 not met. Expected exactly 3 calls, but got 2., Expectation x == 2 not met. Expected exactly 3 calls, but got 2.]"
        );
    }

    #[test]
    fn test_in_sequence_success() {
        let mut expectations = CallExpectations::new();
        expectations.in_sequence = true;
        expectations.add_expectation(
            |&x| x == 1,
            CallRange::new(Bound::Included(1), Bound::Included(2)),
            "x == 1"
        );
        expectations.add_expectation(
            |&x| x == 2,
            CallRange::new(Bound::Included(1), Bound::Included(2)),
            "x == 2"
        );

        expectations.record_call(&1).unwrap();
        expectations.record_call(&1).unwrap();
        expectations.record_call(&2).unwrap();
        expectations.record_call(&2).unwrap();

        assert_eq!(expectations.calls.len(), 0); // All expectations should be satisfied and removed from the queue

        assert!(expectations.is_met().is_ok());
    }

    #[test]
    fn test_in_sequence_failure_wrong_order() {
        let mut expectations = CallExpectations::new();
        expectations.in_sequence = true;
        expectations.add_expectation(
            |&x| x == 1,
            CallRange::new(Bound::Included(1), Bound::Included(2)),
            "x == 1"
        );
        expectations.add_expectation(
            |&x| x == 2,
            CallRange::new(Bound::Included(1), Bound::Included(2)),
            "x == 2"
        );
        expectations.add_expectation(
            |&x| x == 3,
            CallRange::new(Bound::Included(1), Bound::Included(2)),
            "x == 3"
        );

        expectations.record_call(&1).unwrap();
        expectations.record_call(&2).unwrap();
        let res = expectations.record_call(&1); // This call is out of sequence, since the first expectation expects a call with argument 1, but we already satisfied it and moved on to the next expectation which expects a call with argument 2

        assert!(res.is_err());
        let error_message = res.err().unwrap();
        assert_eq!(error_message, "x == 3 is not valid for 1. Expected in 1..=2 calls, but got 0");
    }

    #[test]
    fn test_in_sequence_failure_wrong_first_call() {
        let mut expectations = CallExpectations::new();
        expectations.set_in_sequence();
        expectations.add_expectation(
            |&x| x == 1,
            CallRange::new(Bound::Included(1), Bound::Included(2)),
            "x == 1"
        );
        expectations.add_expectation(
            |&x| x == 2,
            CallRange::new(Bound::Included(1), Bound::Included(2)),
            "x == 2"
        );

        let result = expectations.record_call(&2); // This call is out of sequence, since the first expectation expects a call with argument 1
        assert!(result.is_err());
        let error_message = result.err().unwrap();
        assert_eq!(error_message, "x == 1 is not valid for 2. Expected in 1..=2 calls, but got 0");
    }

    #[test]
    fn test_in_sequence_failure_not_enough_calls() {
        let mut expectations = CallExpectations::new();
        expectations.set_in_sequence();
        expectations.add_expectation(
            |&x| x == 1,
            CallRange::new(Bound::Included(2), Bound::Included(2)),
            "x == 1"
        );
        expectations.add_expectation(
            |&x| x == 2,
            CallRange::new(Bound::Included(1), Bound::Included(2)),
            "x == 2"
        );

        expectations.record_call(&1).unwrap(); // Only one call with argument 1, but the expectation requires exactly 2

        let result = expectations.is_met();
        assert!(result.is_err());
        let error_message = result.err().unwrap();
        assert_eq!(
            error_message,
            "Not all expectations were satisfied. Next expectation x == 1 has 1 calls, expected exactly 2 calls."
        );
    }
}
