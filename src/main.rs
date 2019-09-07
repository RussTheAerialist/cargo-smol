use serde_json::from_str;
use std::process::Command;

mod parse;
mod count;
mod failed;

use parse::*;
use count::*;
use failed::*;

fn feed(count: &mut TestCount, failed: &mut FailedTests, new_results: &TestResult) {
    count.feed(new_results);
    failed.feed(new_results);
}

fn main() {
    let testproc = Command::new("cargo")
        .arg("test")
        .arg("--")
        .arg("--format")
        .arg("json")
        .arg("-Z")
        .arg("unstable-options")
        .output()
        .expect("Unable to run cargo test");

    let lines = std::str::from_utf8(&testproc.stdout).expect("Unable to process output, non-utf8 characters outputted");
    let mut failed_tests = FailedTests::default();
    let results = lines.split("\n").fold(TestCount::default(), |mut acc, line| {
        if let Ok(result) = from_str::<TestResult>(&line) {
            feed(&mut acc, &mut failed_tests, &result);
        }
        acc
    });
    println!("{:?}\n{:#?}", results, failed_tests);
}
