#![no_std] 
#![no_main]

use core::panic::PanicInfo;

// Définitions des ports et des registres
const PORTB: *mut u8 = 0x25 as *mut u8; // Adresse du registre PORTB
const DDRB: *mut u8 = 0x24 as *mut u8; // Adresse du registre DDRB
const PINB: *const u8 = 0x23 as *const u8; // Adresse du registre PINB

// Fonction d'interruption de panique
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Configure la broche (pin) comme entrée
fn pin_mode_input(pin: u8) {
    unsafe {
        *DDRB &= !(1 << pin); // met le bit correspondant dans DDRB à 0
    }
}

/// Configure la broche (pin) comme sortie
fn pin_mode_output(pin: u8) {
    unsafe {
        *DDRB |= 1 << pin; // met le bit correspondant dans DDRB à 1
    }
}

/// Lire l'état de la broche (pin).
fn digital_read(pin: u8) -> bool {
    unsafe {
        (*PINB & (1 << pin)) != 0 // Retourne true si le bit est à 1 (HIGH), sinon false (LOW)
    }
}

/// Écrire une valeur sur la broche (pin).
fn digital_write(pin: u8, value: bool) {
    unsafe {
        if value {
            *PORTB |= 1 << pin; // Si la valeur est true, met le bit à 1 (HIGH)
        } else {
            *PORTB &= !(1 << pin); // Si la valeur est false, met le bit à 0 (LOW)
        }
    }
}





/* FONCTION EXEMPLE
Pour l'exemple, nous allons configurer la broche 2 comme sortie et la mettre à un état haut
Nous allons également configurer la broche 3 comme entrée et lire son état
*/

#[no_mangle]
pub extern "C" fn main() -> ! {
    pin_mode_output(2); // Configure la broche 2 comme sortie
    digital_write(2, true); // Met la broche 2 à l'état haut

    pin_mode_input(3); // Configure la broche 3 comme entrée
    let _state = digital_read(3); // Lit l'état de la broche 3 sans utiliser la variable

    loop {} // Ajoutez une boucle infinie ici
}