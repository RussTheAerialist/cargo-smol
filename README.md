# The smallest test reporter ever

`cargo-smol` takes the output of `cargo test --format json` and formats it into a small area.

_note: `smol` requires `-Z unstable-options` until the json format is stablized._

Typically, `smol` is paired with `watch` so that it gets rerun whenever a file changes. Try this in your project: `cargo watch -x smol`.

If all of your tests are passing, this should show up:

![screenshot of smol in action]()

If you have a test failure, you should see something like this:

![screenshot of smol with a couple of failed test cases]()

`smol` will only show as many test cases as can fit into the size of the window it's in. Any additional tests will not be listed, but will be
included in the test summary status line.

## Current Release: v0.1.1

# Technical Details

Something about the way `cargo-watch` works inhibits the ability to switch the terminal into raw mode. Because of this, we cannot have complete
control over the terminal. We do our best, but there are probably bugs.

I know a couple of the test cases are currently failing.
