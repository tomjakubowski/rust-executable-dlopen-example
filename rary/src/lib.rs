#[no_mangle]
pub extern "C" fn hello() {
    println!("hello from library");
}
