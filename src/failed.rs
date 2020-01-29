use std::ops::AddAssign;

use crate::parse::*;

#[derive(Debug, Default)]
pub(super) struct FailedTests(pub(crate) Vec<String>);

impl FailedTests {
    pub(super) fn feed(&mut self, result: &TestResult) {
        *self += result
    }
}

impl AddAssign<&TestResult> for FailedTests {
   fn add_assign(&mut self, rhs: &TestResult) {
       match rhs {
           TestResult::Test { event, name } => {
               match event {
                   TestEvent::Failed { test_output } => {
                       self.0.push(name.clone());
                   },
                   _ => { }
               };
           },
           _ => { }
   };
   }
}

mod tests {
    #[test]
    fn failing_test() {
//        assert!(false);
    }
}
