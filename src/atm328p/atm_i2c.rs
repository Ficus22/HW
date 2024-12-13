// I2C Registers
const TWBR: *mut u8 = 0xB8 as *mut u8; // TWI Bit Rate Register
const TWSR: *mut u8 = 0xB9 as *mut u8; // TWI Status Register
const TWCR: *mut u8 = 0xBC as *mut u8; // TWI Control Register
const TWDR: *mut u8 = 0xBB as *mut u8; // TWI Data Register
const TWAR: *mut u8 = 0xBA as *mut u8; // TWI (Slave) Address Register
const TWPS_MASK: u8 = 0b00000011;      // Prescaler bits mask in TWSR

// Initialisation de l'I2C (mode maître)
pub fn i2c_init(clock_speed: u32, cpu_freq: u32) {
    unsafe {
        let twbr_value = ((cpu_freq / clock_speed) - 16) / 2;
        *TWBR = twbr_value as u8; // Configurer la vitesse du bus I2C
        *TWSR &= !TWPS_MASK;      // Configurer le prescaler à 1
    }
}

// Démarrage de l'I2C
pub fn i2c_start() {
    unsafe {
        *TWCR = (1 << 7) | (1 << 5) | (1 << 2); // TWINT | TWSTA | TWEN
        while (*TWCR & (1 << 7)) == 0 {}        // Attendre la fin de l'opération
    }
}

// Arrêt de l'I2C
pub fn i2c_stop() {
    unsafe {
        *TWCR = (1 << 7) | (1 << 4) | (1 << 2); // TWINT | TWSTO | TWEN
    }
}

// Envoi d'un octet via I2C
pub fn i2c_write(data: u8) {
    unsafe {
        *TWDR = data;                           // Charger les données
        *TWCR = (1 << 7) | (1 << 2);            // TWINT | TWEN
        while (*TWCR & (1 << 7)) == 0 {}        // Attendre la fin de l'opération
    }
}

// Réception d'un octet via I2C 
pub fn i2c_read(ack: bool) -> u8 {
    unsafe {
        if ack {
            *TWCR = (1 << 7) | (1 << 2) | (1 << 6); // TWINT | TWEN | TWEA (ACK)
        } else {
            *TWCR = (1 << 7) | (1 << 2);            // TWINT | TWEN (NACK)
        }
        while (*TWCR & (1 << 7)) == 0 {}            // Attendre la fin de l'opération
        *TWDR // Retourner les données reçues
    }
}
