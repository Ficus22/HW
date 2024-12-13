#![no_std] 
#![no_main]

use core::panic::PanicInfo;

#[cfg(feature = "esp32")]
mod esp32;

#[cfg(feature = "esp32")]
use esp32::fn_ex;


#[cfg(feature = "atmega")]
mod atm328p;

#[cfg(feature = "atmega")]
use atm328p::fn_ex;




// Fonction d'interruption de panique
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main()->!{
    fn_ex();
    loop {
    }
}