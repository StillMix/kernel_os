#![no_std]
#![no_main]

mod boot;
mod memory;
mod interrupts;
mod io;

use core::panic::PanicInfo;

// Правильный Multiboot заголовок для GRUB
#[repr(C, packed)]
struct MultibootHeader {
    magic: u32,
    flags: u32,
    checksum: u32,
}

const MULTIBOOT_MAGIC: u32 = 0x1BADB002;
const MULTIBOOT_FLAGS: u32 = 0x0;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".multiboot_header")]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: MULTIBOOT_MAGIC,
    flags: MULTIBOOT_FLAGS,
    checksum: (0u32.wrapping_sub(MULTIBOOT_MAGIC).wrapping_sub(MULTIBOOT_FLAGS)),
};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    boot::init();
    
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}