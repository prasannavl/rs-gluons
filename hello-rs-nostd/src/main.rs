#![feature(lang_items, core_intrinsics)]
#![feature(start)]
#![no_std]

extern crate winapi;
extern crate kernel32;

use core::ptr;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let msg = "Hello world!\n";
    let stdout = unsafe { kernel32::GetStdHandle(winapi::STD_OUTPUT_HANDLE) };
    unsafe { kernel32::WriteConsoleA(stdout, msg.as_ptr() as *const winapi::VOID, msg.len() as winapi::DWORD, ptr::null_mut(), ptr::null_mut()) };
    0
}