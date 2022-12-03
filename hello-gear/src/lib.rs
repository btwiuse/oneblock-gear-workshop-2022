#![no_std]

#[no_mangle]
extern "C" fn handle() {
  let _ = gstd::msg::load_bytes(); // read input message and do nothing
  gstd::msg::reply_bytes(gstd::String::from("Hello world!"), 0);
}
