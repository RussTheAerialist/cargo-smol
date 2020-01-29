use std::ops::AddAssign;
use std::fmt::{Display, Formatter, self};

use crate::parse::*;

#[derive(Debug, Default, Clone, Copy)]
pub(super) struct TestCount {
    pub ran_count: u32,
    pub failed_count: u32,
}

impl TestCount {
    pub(super) fn feed(&mut self, result: &TestResult) {
        *self += result;
    }

    pub(super) fn was_successful(&self) -> bool {
        self.failed_count == 0 && self.ran_count > 0
    }
}

impl Display for TestCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.was_successful() {
            write!(f, "{} tests passed", self.ran_count)
        } else {
            write!(f, "{}/{} tests failed", 
              self.failed_count, self.ran_count
            )
        }
    }
}

impl AddAssign<&TestResult> for TestCount {
   fn add_assign(&mut self, rhs: &TestResult) {
       match rhs { 
           TestResult::Test { event, name } => {
               match event { 
                   TestEvent::Started => { self.ran_count += 1; },
                   TestEvent::Failed { test_output: _ } => { self.failed_count += 1; },
                   _ => { },
               }
           },
           _ => { },
       }
   }
}

pub(crate) mod tests {
    use super::*;
    use crate::failed::FailedTests;
    use crate::feed;
    use serde_json::from_str;

    pub const TEST_STARTED : & str = r#"{ "type": "test", "event": "started", "name": "tests::we_dont_use_this_value" }"#;
    pub const TEST_OK : &str = r#"{ "type": "test", "name": "tests::test_ok_parsed_properly", "event": "ok" }"#;
    pub const TEST_FAILED: &str = r#"
    { "type": "test", "name": "tests::test_ok_parsed_properly", "event": "failed",
      "stdout": "thread 'main' panicked at 'assertion failed: `(left == right)`\n  left: `\"tests::test_ok_parsed_properly\"`,\n right: `\"\"`', src/main.rs:117:34\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.\n" }
    "#;

    #[test]
    fn test_ok_works() {
        let started : TestResult =from_str(TEST_STARTED).unwrap();
        let result : TestResult = from_str(TEST_OK).unwrap();
        let mut count = TestCount::default();
        let mut failed = FailedTests::default();

        feed(&mut count, &mut failed, &started); 
        feed(&mut count, &mut failed, &result); 

        println!("{:#?}", count);
        assert!(count.was_successful());
        assert_eq!(count.failed_count, 0);
    }

    #[test]
    fn test_failed_works() {
        let started : TestResult =from_str(TEST_STARTED).unwrap();
        let result : TestResult = from_str(TEST_FAILED).unwrap();
        let mut count = TestCount::default();
        let mut failed = FailedTests::default();

        feed(&mut count, &mut failed, &started); 
        feed(&mut count, &mut failed, &result); 

        println!("{:#?}", count);
        assert!(!count.was_successful());
        assert_eq!(count.failed_count, 1);
    }
}
