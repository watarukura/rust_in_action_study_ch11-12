#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#[allow(unused)]

use core::intrinsics::{abort};
use core::panic::PanicInfo;

use x86_64::instructions::{hlt};

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        abort();
    }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() { }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;

    unsafe {
        framebuffer.offset(1).write_volatile(0x30);
    }

    loop {
        hlt();
    }
}

// struct VGACell {
//     is_blinking: u1,
//     backglound_color: u3,
//     is_bright: u1,
//     character_color: u3,
//     character: u8,
// }

#[derive(Clone, Copy)]
#[repr(u8)]
enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    Gray = 7,
    White = 8,
    BrightBlue = 9,
    BrightGreen = 10,
    BrightCyan = 11,
    BrightRed = 12,
    BrightMagenta = 13,
    Yellow = 14,
    DarkGray = 15,
}

struct Cursor {
    position: isize,
    foreground: Color,
    background: Color,
}

impl Cursor {
    fn color(&self) -> u8 {
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;
        fg | bg
    }

    fn print(&mut self, text: &[u8]) {
        let color = self.color();

        let framebuffer = 0xb8000 as *mut u8;

        for &character in text {
            unsafe {
                framebuffer.offset(self.position).write_volatile(character);
                framebuffer.offset(self.position + 1).write_volatile(color);
            }
            self.position += 2;
        }
    }
}
