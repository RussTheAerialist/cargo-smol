#![allow(unused_variables)]
use std::ops::AddAssign;

use crate::parse::*;

#[derive(Debug, Default, Clone, Copy)]
pub(super) struct TestCount {
    pub suite_count: u32,
    pub suite_fail_count: u32,
    pub ran_count: u32,
    pub failed_count: u32,
}

impl TestCount {
    pub(super) fn feed(&mut self, result: &TestResult) {
        *self += result;
    }
}

impl AddAssign<&TestResult> for TestCount {
   fn add_assign(&mut self, rhs: &TestResult) {
       match rhs { 
           TestResult::Suite { event } => {
               match event {
                   SuiteEvent::Started { test_count: _ } => {},
                   SuiteEvent::Passed { counts } => {
                       self.suite_count += 1;
                   },
                   SuiteEvent::Failed { counts } => {
                       self.suite_count += 1;
                       self.suite_fail_count += 1;
                   },
               }
           },
           TestResult::Test { event, name } => {
               match event { 
                   TestEvent::Started => { self.ran_count += 1; },
                   TestEvent::Failed { test_output: _ } => { self.failed_count += 1; },
                   _ => { },
               }
           },
       }
   }
}

pub(crate) mod tests {
    use super::*;
    use crate::failed::FailedTests;
    use crate::feed;
    use serde_json::from_str;

    pub const SUITE_STARTED : &str = r#"{ "type": "suite", "event": "started", "test_count": 0 }"#;
    pub const SUITE_FINISHED_OK : &str = r#"{ "type": "suite", "event": "ok", "passed": 1, "failed": 0, "allowed_fail": 0, "ignored": 2, "measured": 0, "filtered_out": 0 }"#;
    pub const SUITE_FINISHED_FAILED : &str = r#"{ "type": "suite", "event": "failed", "passed": 2, "failed": 1, "allowed_fail": 0, "ignored": 0, "measured": 0, "filtered_out": 0 }"#;

    pub const TEST_OK : &str = r#"{ "type": "test", "name": "tests::test_ok_parsed_properly", "event": "ok" }"#;
    pub const TEST_FAILED: &str = r#"
    { "type": "test", "name": "tests::test_ok_parsed_properly", "event": "failed",
      "stdout": "thread 'main' panicked at 'assertion failed: `(left == right)`\n  left: `\"tests::test_ok_parsed_properly\"`,\n right: `\"\"`', src/main.rs:117:34\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.\n" }
    "#;

    #[test]
    fn suite_finished_adds_to_test_count() {
        let result : TestResult = from_str(SUITE_FINISHED_OK).unwrap();
        let mut count = TestCount::default();
        let mut failed = FailedTests::default();
        feed(&mut count, &mut failed, &result);
        assert_eq!(count.ran_count, 1);
        assert_eq!(count.suite_count, 3);
    }
}
