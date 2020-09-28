#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(optin_builtin_traits)]
#![feature(raw_vec_internals)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;

<<<<<<< HEAD
const GPIO_BASE: usize = 0xFE000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

=======
extern crate alloc;

pub mod allocator;
>>>>>>> skeleton/lab3
pub mod console;
pub mod fs;
pub mod mutex;
pub mod shell;

use console::kprintln;
use pi::timer::spin_sleep;
use pi::uart::MiniUart;
use pi::gpio::*;
use core::time::Duration;
use shim::io::{Read, Write as IOWrite};
use core::fmt::Write;

<<<<<<< HEAD
fn blink_test() {
    let mut pin16 = Gpio::<Uninitialized>::new(16).into_output();
    loop {
        pin16.set();
        spin_sleep(Duration::new(1, 0));
        pin16.clear();
        spin_sleep(Duration::new(1, 0));
    }
}

fn kmain() -> ! {
    // FIXME: Start the shell.
=======
use allocator::Allocator;
use fs::FileSystem;

#[cfg_attr(not(test), global_allocator)]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();
pub static FILESYSTEM: FileSystem = FileSystem::uninitialized();

fn kmain() -> ! {
    unsafe {
        ALLOCATOR.initialize();
        FILESYSTEM.initialize();
    }

    kprintln!("Welcome to cs3210!");
>>>>>>> skeleton/lab3
    shell::shell("> ");
}
