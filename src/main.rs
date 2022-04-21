#![no_std]
#![no_main]

use core::ptr;
mod static_ref;

mod cache;
mod descr;
mod nand;
mod panic;
mod uart;
mod usb;
use core::arch::global_asm;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    // Initialize RAM
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    main();

    loop {}
}

global_asm!(include_str!("start.s"));

pub fn debug(text: &str) {
    for byte in text.bytes() {
        if byte == b'\n' {
            uart::write_byte(b'\r');
        }
        uart::write_byte(byte);
    }
}

pub fn debug_hex32(val: u32) {
    for i in (0..8).rev() {
        uart::write_byte(b"0123456789ABCDEF"[((val >> (i * 4)) % 16) as usize]);
    }
}

pub fn debug_hex8(val: u8) {
    for i in (0..2).rev() {
        uart::write_byte(b"0123456789ABCDEF"[((val >> (i * 4)) % 16) as usize]);
    }
}

fn main() -> () {
    debug("AArch64 Bare Metal, 2.0!\n");

    debug("NAND init...\n");
    unsafe {
        core::ptr::write_volatile(0xE001A050 as *mut u32, 0);
    }

    nand::nand_init();
    usb::usb_test();
}
