// Définitions des ports et des registres
const PORTB: *mut u8 = 0x25 as *mut u8; // Adresse du registre PORTB
const DDRB: *mut u8 = 0x24 as *mut u8; // Adresse du registre DDRB
const PINB: *const u8 = 0x23 as *const u8; // Adresse du registre PINB

/// Configure la broche (pin) comme entrée
pub fn pin_mode_input(pin: u8) {
    unsafe {
        *DDRB &= !(1 << pin); // met le bit correspondant dans DDRB à 0
    }
}

/// Configure la broche (pin) comme sortie
pub fn pin_mode_output(pin: u8) {
    unsafe {
        *DDRB |= 1 << pin; // met le bit correspondant dans DDRB à 1
    }
}

/// Lire l'état de la broche (pin).
pub fn digital_read(pin: u8) -> bool {
    unsafe {
        (*PINB & (1 << pin)) != 0 // Retourne true si le bit est à 1 (HIGH), sinon false (LOW)
    }
}

/// Écrire une valeur sur la broche (pin).
pub fn digital_write(pin: u8, value: bool) {
    unsafe {
        if value {
            *PORTB |= 1 << pin; // Si la valeur est true, met le bit à 1 (HIGH)
        } else {
            *PORTB &= !(1 << pin); // Si la valeur est false, met le bit à 0 (LOW)
        }
    }
}