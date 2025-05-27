#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Подключаем модуль для работы с таблицами страниц
//mod page_table;

/// Обработчик паники - вызывается когда программа паникует
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Точка входа - с нее начинается выполнение нашего ядра
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Указатель на буфер видеопамяти (текстовый режим VGA)
    let vga_buffer = 0xb8000 as *mut u8;
    
    // Пишем букву 'H' белым цветом на черном фоне
    unsafe {
        *vga_buffer.offset(0) = b'H';          // символ
        *vga_buffer.offset(1) = 0x0f;          // цвет (белый на черном)
        *vga_buffer.offset(2) = b'e';          // символ
        *vga_buffer.offset(3) = 0x0f;          // цвет
        *vga_buffer.offset(4) = b'l';          // символ
        *vga_buffer.offset(5) = 0x0f;          // цвет
        *vga_buffer.offset(6) = b'l';          // символ
        *vga_buffer.offset(7) = 0x0f;          // цвет
        *vga_buffer.offset(8) = b'o';          // символ
        *vga_buffer.offset(9) = 0x0f;          // цвет
    }
    
    loop {}
}