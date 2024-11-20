#![no_std] 
#![no_main]

use core::panic::PanicInfo;


// Fonction d'interruption de panique
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Code spécifique à l'ATmega328P
#[cfg(feature = "atmega328")]
mod atmega {
    use super::*;

    // Définitions des ports et des registres
    const PORTB: *mut u8 = 0x25 as *mut u8; // Adresse du registre PORTB
    const DDRB: *mut u8 = 0x24 as *mut u8; // Adresse du registre DDRB
    const PINB: *const u8 = 0x23 as *const u8; // Adresse du registre PINB

    // SPI Registers
    const SPCR: *mut u8 = 0x2C as *mut u8; // SPI Control Register
    const SPSR: *mut u8 = 0x2D as *mut u8; // SPI Status Register
    const SPDR: *mut u8 = 0x2E as *mut u8; // SPI Data Register

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

    // Initialisation SPI
    fn spi_init() {
        unsafe {
            *SPCR = (1 << 6) | (1 << 4) | (1 << 5); // Enable SPI, Set as Master, Set Clock Rate fck/16
        }
    }

   // Envoie de la data via SPI
    fn spi_send(data: u8) {
        unsafe {
            *SPDR = data; // Load data into the buffer
            while (*SPSR & (1 << 7)) == 0 {} // Wait until transmission complete
        }
    }

    // Reception de la data via SPI
    fn spi_receive() -> u8 {
        spi_send(0xFF); // Send dummy byte to generate clock
        unsafe { *SPDR } // Read received data from the buffer
    }


    /* FONCTION EXEMPLE
    Pour l'exemple, nous allons configurer la broche 2 comme sortie et la mettre à un état haut
    Nous allons également configurer la broche 3 comme entrée et lire son état
    */

    #[no_mangle]
    pub extern "C" fn main() -> ! {
        pin_mode_output(2); // Configure la broche 2 comme sortie
        pin_mode_input(3); // Configure la broche 3 comme entrée

        spi_init(); // Initialisation SPI
        

        loop {
            digital_write(2, true); // Met la broche 2 à l'état haut
            let _state = digital_read(3); // Lit l'état de la broche 3 sans utiliser la variable

            spi_send(0xA5); // Envoie d'une data exemple
            let received = spi_receive(); // Reception d'une data exemple
        }
    }

}


// Code spécifique à l'ESP32
#[cfg(feature = "esp32")]
mod esp32 {
    use super::*;
    use esp32_hal::{prelude::*, pac::Peripherals, spi::Spi};

    // Définitions des registres GPIO pour l'ESP32
    const GPIO_OUT_REG: *mut u32 = 0x3FF44004 as *mut u32;
    const GPIO_ENABLE_REG: *mut u32 = 0x3FF44020 as *mut u32;
    const GPIO_IN_REG: *const u32 = 0x3FF4403C as *const u32;

    // Fonction pour configurer un GPIO en sortie
    fn set_gpio_output(gpio_num: u32) {
        unsafe {
            // Activer le GPIO en sortie
            *GPIO_ENABLE_REG |= 1 << gpio_num;
        }
    }

    // Fonction pour configurer un GPIO en entrée
    fn set_gpio_input(gpio_num: u32) {
        unsafe {
            // Désactiver le GPIO en sortie (le met en entrée)
            *GPIO_ENABLE_REG &= !(1 << gpio_num);
        }
    }

    // Fonction pour écrire sur un GPIO
    fn gpio_write(gpio_num: u32, value: bool) {
        unsafe {
            if value {
                *GPIO_OUT_REG |= 1 << gpio_num;
            } else {
                *GPIO_OUT_REG &= !(1 << gpio_num);
            }
        }
    }

    // Fonction pour lire l'état d'un GPIO
    fn gpio_read(gpio_num: u32) -> bool {
        unsafe {
            (*GPIO_IN_REG & (1 << gpio_num)) != 0
        }
    }


    /* FONCTION EXEMPLE
    Pour l'exemple, nous allons configurer la broche 2 comme sortie et la mettre à un état haut
    Nous allons également configurer la broche 3 comme entrée et lire son état
    */

    #[entry]
    fn main() -> ! {
        let peripherals = Peripherals::take().unwrap(); // initialisation de l'accès hardware de l'esp32
        let system = peripherals.DPORT.split(); // séparation des DPORT
        let clocks = ClockControl::boot_defaults(system.clock_control).freeze(); // initialisation de la clock

        // Définition des numéros de GPIO pour l'ESP32
        const GPIO_OUTPUT: u32 = 2;  // Utilisation du GPIO2 comme sortie
        const GPIO_INPUT: u32 = 3;   // Utilisation du GPIO3 comme entrée

        // Configuration des GPIO
        set_gpio_output(GPIO_OUTPUT);  // Configure le GPIO2 comme sortie
        set_gpio_input(GPIO_INPUT);    // Configure le GPIO3 comme entrée

        // initialisation d'une nouvelle instance SPI
        let mut spi = Spi::new(
            peripherals.SPI2,
            (GPIO_OUTPUT, GPIO_INPUT), // MISO, MOSI pins
            &clocks,
            spi::MODE_0,
            1.mhz(),
        );


        loop {
            gpio_write(GPIO_OUTPUT, true);  // Met le GPIO2 à l'état haut
            let _state = gpio_read(GPIO_INPUT);  // Lit l'état du GPIO3 sans utiliser la variable

            let _ = spi.write(&[0xA5]); // Envoie d'une data exemple
            let mut buffer = [0; 1];
            let _ = spi.read(&mut buffer); // Reception d'une data exemple
        }
    }
}