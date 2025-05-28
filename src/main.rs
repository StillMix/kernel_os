#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Точка входа. Обозначается `no_mangle`, чтобы компилятор не менял имя функции.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    // Печатаем слово "Привет" в VGA-буфер по байтам.
    let hello = b"Privet"; // Пишем латиницей для совместимости
    for (i, &byte) in hello.iter().enumerate() {
        unsafe {
            // VGA-память: каждый символ состоит из 2 байт — сам символ и цвет
            *vga_buffer.offset((i * 2) as isize) = byte;
            *vga_buffer.offset((i * 2 + 1) as isize) = 0x0f; // белый текст на черном фоне
        }
    }

    loop {}
}

/// Хук на случай паники. Обязателен в no_std.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
