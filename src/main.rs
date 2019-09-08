use serde_json::from_str;
use std::process::Command;
use termion::{clear, color, cursor, style};
use std::io::{Write, stdout, Stdout};
use std::convert::TryInto;

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

fn status(out: &mut Stdout, msg: &str) -> Result<(), std::io::Error> {
    write!(out, "{goto}{fg}{bg}{clear}{msg}{fre}{bre}",
           goto = cursor::Goto(1,1),
           fg = color::Fg(color::White),
           bg = color::Bg(color::Blue),
           clear = clear::CurrentLine,
           msg = msg,
           fre = color::Fg(color::Reset),
           bre = color::Bg(color::Reset)
    )?;
    out.flush()?;
    Ok(())
}

fn output_summary(out: &mut Stdout, result: &TestCount) -> Result<(), std::io::Error> {
    write!(out, "{goto}{fg}{bg}{clear}{msg}{fre}{bre}",
      goto = cursor::Goto(1,1),
      fg = color::Fg(color::White),
      bg = if result.was_successful() { color::Bg(color::Blue).to_string() } else { color::Bg(color::Red).to_string() },
      clear = clear::CurrentLine,
      msg = result,
      fre = color::Fg(color::Reset),
      bre = color::Bg(color::Reset)
    )?;

    Ok(())
}

fn output_failed_tests(out: &mut Stdout, failed: &FailedTests, height: u16) -> Result<(), std::io::Error> {
    write!(out, "{}", cursor::Goto(1, 2))?;
    let test_count_to_display : usize = (height - 2).try_into().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, ""))?;
    failed.0.iter().take(test_count_to_display).for_each(|item| {
        if let(Ok(_)) = writeln!(out, "{}", item) {
        }
    });

    Ok(())
}

fn setup_console() -> Result<Stdout, std::io::Error> {
    let mut console = stdout();
    write!(console, "{}{}", clear::All, cursor::Hide)?;
    Ok(console)
}

fn finish(out: &mut Stdout, height: u16) -> Result<(), std::io::Error> {
    let last_line = height-1;
    write!(out, "{reset}{goto}",
        reset = style::Reset,
        goto = cursor::Goto(1, last_line)
    )?;
    out.flush()?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let mut console = setup_console()?;
    let terminal_size = termion::terminal_size()?;
    let height : u16 = terminal_size.1;

    status(&mut console, "Running Tests...")?;
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
    output_summary(&mut console, &results)?;
    output_failed_tests(&mut console, &failed_tests, height)?;
    finish(&mut console, height)?;

    Ok(())
}
