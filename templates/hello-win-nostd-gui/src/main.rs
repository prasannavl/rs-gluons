#![feature(lang_items, core_intrinsics)]
#![feature(start)]
#![no_std]
#![windows_subsystem = "windows"]

extern crate winapi;
extern crate kernel32;
extern crate user32;

use user32::MessageBoxA;
use winapi::winuser::{MB_OK, MB_ICONINFORMATION};

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let lp_text = "Hello world!\0";
    let lp_caption = "Hello\0";

    unsafe {
        let _ = MessageBoxA(
            core::ptr::null_mut(),
            lp_text.as_ptr() as *const i8,
            lp_caption.as_ptr() as *const i8,
            MB_OK | MB_ICONINFORMATION
        );
    }

    0
}