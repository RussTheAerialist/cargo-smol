use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum TestResult {
    #[serde(rename(deserialize = "suite"))]
    Suite {
        #[serde(flatten)]
        event: SuiteEvent,
    },
    #[serde(rename(deserialize = "test"))]
    Test {
        #[serde(flatten)]
        event: TestEvent,
        name: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "event")]
pub(crate) enum SuiteEvent {
    #[serde(rename(deserialize = "started"))]
    Started { test_count: u32 },
    #[serde(rename(deserialize = "ok"))]
    Passed {
        #[serde(flatten)]
        counts: CommonTestCounts,
    },
    #[serde(rename(deserialize = "failed"))]
    Failed {
        #[serde(flatten)]
        counts: CommonTestCounts,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "event")]
pub(crate) enum TestEvent {
    #[serde(rename(deserialize = "started"))]
    Started,
    #[serde(rename(deserialize = "ok"))]
    Passed,
    #[serde(rename(deserialize = "failed"))]
    Failed {
        #[serde(rename(deserialize = "stdout"))]
        test_output: String,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) struct CommonTestCounts {
    pub(crate) passed: u32,
    pub(crate) failed: u32,
    pub(crate) allowed_fail: u32,
    pub(crate) ignored: u32,
    pub(crate) measured: u32,
    pub(crate) filtered_out: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    const SUITE_STARTED: &str = r#"{ "type": "suite", "event": "started", "test_count": 0 }"#;
    const SUITE_FINISHED_OK: &str = r#"{ "type": "suite", "event": "ok", "passed": 1, "failed": 0, "allowed_fail": 0, "ignored": 2, "measured": 0, "filtered_out": 0 }"#;
    const SUITE_FINISHED_FAILED: &str = r#"{ "type": "suite", "event": "failed", "passed": 2, "failed": 1, "allowed_fail": 0, "ignored": 0, "measured": 0, "filtered_out": 0 }"#;

    const TEST_OK: &str =
        r#"{ "type": "test", "name": "tests::test_ok_parsed_properly", "event": "ok" }"#;
    const TEST_FAILED: &str = r#"
    { "type": "test", "name": "tests::test_ok_parsed_properly", "event": "failed",
      "stdout": "thread 'main' panicked at 'assertion failed: `(left == right)`\n  left: `\"tests::test_ok_parsed_properly\"`,\n right: `\"\"`', src/main.rs:117:34\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.\n" }
    "#;

    #[test]
    fn suite_started_parses_properly() {
        let actual: TestResult = serde_json::from_str(SUITE_STARTED).unwrap();
        match actual {
            TestResult::Suite { event } => match event {
                SuiteEvent::Started { test_count } => assert_eq!(test_count, 0),
                _ => assert!(false),
            },
            _ => assert!(false),
        };
    }

    #[test]
    fn suite_failed_parsed_properly() {
        let actual: TestResult = serde_json::from_str(SUITE_FINISHED_FAILED).unwrap();
        match actual {
            TestResult::Suite { event } => match event {
                SuiteEvent::Failed { counts } => {
                    assert_eq!(counts.passed, 2);
                    assert_eq!(counts.failed, 1);
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn suite_ok_parsed_properly() {
        let actual: TestResult = serde_json::from_str(SUITE_FINISHED_OK).unwrap();
        match actual {
            TestResult::Suite { event } => match event {
                SuiteEvent::Passed { counts } => {
                    assert_eq!(counts.passed, 1);
                    assert_eq!(counts.ignored, 2);
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_ok_parsed_properly() {
        let actual: TestResult = serde_json::from_str(TEST_OK).unwrap();
        match actual {
            TestResult::Test { event, name } => match event {
                TestEvent::Passed => assert_eq!(name, "tests::test_ok_parsed_properly"),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn test_failed_parsed_properly() {
        let actual: TestResult = serde_json::from_str(TEST_FAILED).unwrap();
        match actual {
            TestResult::Test { event, name } => match event {
                TestEvent::Failed { test_output } => {
                    assert_eq!(name, "tests::test_ok_parsed_properly");
                    assert!(test_output.contains("RUST_BACKTRACE=1"));
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }
}
