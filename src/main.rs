#![no_std]
#![no_main]

mod boot;
mod memory;
mod interrupts;
mod io;

use core::panic::PanicInfo;

// Multiboot заголовок
#[unsafe(no_mangle)]
#[unsafe(link_section = ".multiboot_header")]
static MULTIBOOT_HEADER: [u32; 12] = [
    0x1BADB002,          // magic
    0x00,                // flags
    0xE4524FFE,          // checksum
    0, 0, 0, 0, 0, 0, 0, 0, 0
];

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    boot::init();
    
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}