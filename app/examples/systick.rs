#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::peripheral::{syst, Peripherals};
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("Begin").unwrap();

    let mut peripherals = Peripherals::take().unwrap();
    let mut systick = peripherals.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(1_000);
    systick.clear_current();
    systick.enable_counter();
    while !systick.has_wrapped() {
        // Loop
    }

    hprintln!("Looping ...").unwrap();
    loop {}
}
