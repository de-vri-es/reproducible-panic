# reproducible-panic

A panic hook that mimics the default panic hook, but without printing non-reproducible information.

This is useful for snapshot tests where you compare the output of a program to verify it is still functioning correct.
If the program panics, the default hook includes the ID of the panicking thread, which is different on every run.

Rather than trying to filter it out, you can have the program install this panic hook to prevent it from being printed in the first place.

## Example

```rust
fn main() {
  reproducible_panic::install();
  panic!("Oh no!");
}
```

Produces the following output:

```
thread 'main' panicked at examples/example.rs:3:5
Oh no!
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

In contrast, with the default panic hook the first line would look like this:

```
thread 'main' (12993) panicked at examples/example.rs:3:5:
```

Note the "12993" in the output. This number will be different every time you run the program, ruining your snapshot tests.
