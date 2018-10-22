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
use pi::timer::{spin_sleep_ms};

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    let pin16 = Gpio::new(16);
    let pin05 = Gpio::new(5);
    let pin06 = Gpio::new(6);
    let pin13 = Gpio::new(13);
    let pin21 = Gpio::new(21);
    let pin20 = Gpio::new(20);

    let mut pin16 = pin16.into_output();
    let mut pin05 = pin05.into_output();
    let mut pin06 = pin06.into_output();
    let mut pin13 = pin13.into_output();
    let mut pin21 = pin21.into_output();
    let mut pin20 = pin20.into_output();

    loop {
        pin16.set();
        spin_sleep_ms(250);
        pin16.clear();
        spin_sleep_ms(250);

        pin05.set();
        spin_sleep_ms(250);
        pin05.clear();
        spin_sleep_ms(250);

        pin06.set();
        spin_sleep_ms(250);
        pin06.clear();
        spin_sleep_ms(250);

        pin13.set();
        spin_sleep_ms(250);
        pin13.clear();
        spin_sleep_ms(250);

        pin21.set();
        spin_sleep_ms(250);
        pin21.clear();
        spin_sleep_ms(250);

        pin20.set();
        spin_sleep_ms(250);
        pin20.clear();
        spin_sleep_ms(250);
    }
}
