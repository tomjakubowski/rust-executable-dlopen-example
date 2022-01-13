//! Usage:
//!
//! To load the rary crate's dylib:
//!
//! ```
//! cargo run --bin nary -- target/debug/library.so
//! ```

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let libpath = args.get(1).expect("usage: cargo run -- path/to/library");
    unsafe {
        let lib = libloading::Library::new(&libpath)
            .unwrap_or_else(|err| panic!("loading {}: {}", libpath, err));

        let hello: libloading::Symbol<unsafe extern "C" fn() -> ()> =
            lib.get(b"hello").expect("failed loading symbol");

        hello();
    }
}
