use std::ops::Bound;

use crate::spy::range::CallRange;

pub struct TimesExpectation {
    call_range: CallRange,
    times_called: usize,
}

impl TimesExpectation {
    pub fn new(call_range: CallRange) -> Self {
        TimesExpectation {
            call_range,
            times_called: 0,
        }
    }

    pub fn increment_times_called(&mut self) {
        self.times_called += 1;
    }

    pub fn is_met(&self) -> Result<(), String> {
        if self.call_range.is_within(self.times_called) {
            Ok(())
        } else {
            Err(format!("Expected {}, got {}", self.call_range, self.times_called))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_expectation() {
        let mut expectation = TimesExpectation::new(
            CallRange::new(Bound::Included(2), Bound::Included(4))
        );
        expectation.increment_times_called();
        expectation.increment_times_called();
        expectation.increment_times_called();
        assert!(expectation.is_met().is_ok());
    }

    #[test]
    fn test_failed_expectation() {
        let mut expectation = TimesExpectation::new(
            CallRange::new(Bound::Included(2), Bound::Included(4))
        );
        expectation.increment_times_called();
        let result = expectation.is_met();
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Expected in 2..=4 calls, got 1");
    }
}
