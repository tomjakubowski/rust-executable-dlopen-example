rust-executable-dlopen-example
=====

Two crates are in this workspace:
  * `rary` - a library crate built as a dylib
  * `nary` - a binary/executable crate, which dlopen()s `rary` from a
    user-provided path and calls it

Build
------

```
$ cargo build
```

This builds both crates in the workspace in debug mode.

Run
---

```
$ cargo run --bin nary -- target/debug/library.so
...
hello from library!
```

This builds the nary crate (if needed), then executes it, passing a path to the
library artifact as `argv[1]`.  The program dlopen()s the library using the
portable wrapper crate [libloading][] (with an unsafe interface), loads the
symbol `hello` with a type cast, and calls it.

[libloading]: https://docs.rs/libloading/latest/libloading/
