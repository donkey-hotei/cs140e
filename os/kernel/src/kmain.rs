#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]

extern crate pi;
extern crate stack_vec;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

use pi::gpio::Gpio;
use pi::timer::spin_sleep_ms;
use console::{CONSOLE, kprintln};

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
        spin_sleep_ms(100);
    }

    let mut indicator_led = Gpio::new(16).into_output();


    loop {
        let byte = CONSOLE.lock().read_byte();

        kprintln!("Welcome to the Galaxy.");

        indicator_led.set();
        spin_sleep_ms(25);
        indicator_led.clear();
    }
}
