#![no_std]
#![no_main]
use core::panic::PanicInfo;

mod vga_buffer;


#[no_mangle]
pub extern "C" fn _start() -> ! {
   println!("Hello World{}", "!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}