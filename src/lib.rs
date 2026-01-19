//! A panic hook that mimics the default panic hook, but without printing non-reproducible information.
//!
//! This is useful for snapshot tests where you compare the output of a program to verify it is still functioning correct.
//! If the program panics, the default hook includes the ID of the panicking thread, which is different on every run.
//!
//! Rather than trying to filter it out, you can have the program install this panic hook to prevent it from being printed in the first place.
//!
//! # Example
//!
//! ```rust,should_panic
//! fn main() {
//!   reproducible_panic::install();
//!   panic!("Oh no!");
//! }
//! ```
//!
//! Produces the following output:
//!
//! ```text
//! thread 'main' panicked at examples/example.rs:3:5
//! Oh no!
//! note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//! ```
//!
//! In contrast, with the default panic hook the first line would look like this:
//!
//! ```text
//! thread 'main' (12993) panicked at examples/example.rs:3:5:
//! ```
//!
//! Note the "12993" in the output. This number will be different every time you run the program, ruining your snapshot tests.

#![allow(clippy::needless_doctest_main, reason = "included to show intended use in a full program")]

use std::panic::PanicHookInfo;
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::Write;

/// Install [`panic_hook()`] as the global panic hook.
pub fn install() {
	std::panic::set_hook(Box::new(panic_hook));
}

/// A panic hook that doesn't print any non-reproducible information by default.
///
/// The hook tries to mimic the default hook, except that it does not print non-reproducible information like the ID of the panicking thread by default.
///
/// However, if you set `RUST_BACKTRACE=full`, the printed backtrace will almost certainly include non-reproducible output.
pub fn panic_hook(info: &PanicHookInfo<'_>) {
	let backtrace = std::backtrace::Backtrace::capture();
	let location = info.location();
	let msg = info.payload_as_str();
	let current_thread = std::thread::current();
	let thread_name = current_thread.name().unwrap_or("<unnamed>");
	let mut stderr = std::io::stderr().lock();


	if let Some(location) = location {
		writeln!(stderr, "\nthread '{thread_name}' panicked at {location}").ok();
	} else {
		writeln!(stderr, "\nthread '{thread_name}' panicked").ok();
	}
	if let Some(msg) = msg {
		writeln!(stderr, "{msg}").ok();
	}

	static FIRST_PANIC: AtomicBool = AtomicBool::new(true);

	match backtrace.status() {
		std::backtrace::BacktraceStatus::Captured => {
			if std::env::var_os("RUST_BACKTRACE").is_some_and(|x| x == "full") {
				writeln!(&mut stderr, "stack backtrace:\n{backtrace:#}").ok();
			} else {
				writeln!(&mut stderr, "stack backtrace:\n{backtrace}").ok();
			}
		}
		std::backtrace::BacktraceStatus::Disabled => {
			if FIRST_PANIC.swap(false, Ordering::Relaxed) {
				writeln!(
					&mut stderr,
					"note: run with `RUST_BACKTRACE=1` environment variable to display a \
					backtrace"
				).ok();
				if cfg!(miri) {
					writeln!(
						&mut stderr,
						"note: in Miri, you may have to set `MIRIFLAGS=-Zmiri-env-forward=RUST_BACKTRACE` \
						for the environment variable to have an effect"
					).ok();
				}
			}
		}
		std::backtrace::BacktraceStatus::Unsupported => (),
		_ => (),
	}
}
