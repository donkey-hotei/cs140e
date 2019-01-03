#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(panic_info_message)]

extern crate pi;
extern crate core;
extern crate stack_vec;

pub mod allocator;
pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

use allocator::Allocator;
use console::kprintln;
use pi::gpio::Gpio;
use pi::timer::spin_sleep_ms;
use pi::atags::Atags;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

fn run_shell() {
    shell::shell("λ >>");
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    let mut loading_leds = [
        Gpio::new(5).into_output(),
        Gpio::new(6).into_output(),
        Gpio::new(13).into_output(),
        Gpio::new(19).into_output(),
        Gpio::new(26).into_output()
    ];

    for ref mut led in loading_leds.iter_mut() {
        led.set();
        spin_sleep_ms(150);
    }

    let mut indicator_led = Gpio::new(16).into_output();

    indicator_led.set();
    spin_sleep_ms(50);
    indicator_led.clear();

    kprintln!("Welcome to the Galaxy.");

    for atag in Atags::get() { 
        if let Some(mem) = atag.mem() {
            kprintln!("{:#?}", mem);
        }
    }

    ALLOCATOR.initialize();

    loop { run_shell(); }
}
