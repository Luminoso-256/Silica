#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(silica::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use silica::println;



#[no_mangle]
pub extern "C" fn _start() -> ! {
    silica::init();



    #[cfg(test)]
        test_main();
    println!("Silica");
    println!("Enter command:->");
    silica::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    silica::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    silica::test_panic_handler(info)
}
