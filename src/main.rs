#![no_std]
#![no_main]
#[no_mangle]
extern "C" fn rust_eh_personality() {}

#[allow(non_snake_case)]
#[no_mangle]
extern "C" fn _Unwind_Resume() {}

extern crate alloc;
use ::alloc::boxed::Box;
use ::core::{ffi::c_int, panic::PanicInfo};
use libc_alloc::LibcAlloc;
use libc_print::std_name::{dbg, eprintln, println};

#[global_allocator]
static GLOBAL_ALLOC: LibcAlloc = LibcAlloc;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("[FATAL]: {info}");
    unsafe { libc::abort() }
}

use core::str::from_utf8;
use bytes::{Bytes, BytesMut, Buf, BufMut};

#[no_mangle]
pub extern "C" fn main() -> c_int {
    let upper = "ok".to_uppercase();
    println!("{upper}");

    let b = Box::new("heap");
    dbg!(b);

    // fill the buffer
    let mut buffer = [0u8; 32];
    buffer[..6].copy_from_slice(b"IBELL\0");
    buffer[6..8].copy_from_slice(&(50u16.to_be_bytes())[..]);
    println!("{buffer:?}");

    // get back the values from the buffer
    let ibell = from_utf8(&buffer[0..5]).unwrap();
    assert!(ibell == "IBELL");
    let value = u16::from_be_bytes(buffer[6..8].try_into().unwrap());
    println!("{value}");

    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);
    buf.put_u16(1234);
    println!("{buf:?}");

    0
}
