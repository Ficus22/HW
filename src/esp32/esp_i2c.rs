// I2C Registers (ESP32, I2C0)
const I2C_CTR_REG: *mut u32 = 0x60013000 as *mut u32;      // Control Register
const I2C_FIFO_REG: *mut u32 = 0x60013004 as *mut u32;     // FIFO Data Register
const I2C_CMD_REG: *mut u32 = 0x60013020 as *mut u32;      // Command Register
const I2C_STATUS_REG: *mut u32 = 0x60013024 as *mut u32;   // Status Register
const I2C_SCL_LOW_REG: *mut u32 = 0x60013010 as *mut u32;  // SCL Low Period
const I2C_SCL_HIGH_REG: *mut u32 = 0x60013014 as *mut u32; // SCL High Period

// Initialisation de l'I2C (mode maître)
pub fn i2c_init(clock_speed: u32, cpu_freq: u32) {
    unsafe {
        let scl_period = cpu_freq / clock_speed;
        *I2C_SCL_LOW_REG = scl_period / 2;  // Configurer la période basse de SCL
        *I2C_SCL_HIGH_REG = scl_period / 2; // Configurer la période haute de SCL
        *I2C_CTR_REG = 1;                   // Activer l'I2C
    }
}

// Envoi de données via I2C
pub fn i2c_send(data: &[u8], address: u8) {
    unsafe {
        // Charger l'adresse du périphérique avec le bit d'écriture (0)
        *I2C_FIFO_REG = (address as u32) << 1;

        // Charger les données à envoyer dans le FIFO
        for &byte in data {
            *I2C_FIFO_REG = byte as u32;
        }

        // Démarrer la transmission
        *I2C_CMD_REG = 1; // Commande de début de transmission

        // Attendre la fin de la transmission
        while (*I2C_STATUS_REG & 1) != 0 {}
    }
}

// Réception de données via I2C
pub fn i2c_receive(length: usize, address: u8) -> Vec<u8> {
    let mut data = Vec::new();
    unsafe {
        // Charger l'adresse du périphérique avec le bit de lecture (1)
        *I2C_FIFO_REG = ((address as u32) << 1) | 1;

        // Démarrer la réception
        *I2C_CMD_REG = 2; // Commande de début de réception

        // Lire les données reçues
        for _ in 0..length {
            while (*I2C_STATUS_REG & 1) != 0 {} // Attendre que les données soient disponibles
            data.push((*I2C_FIFO_REG & 0xFF) as u8);
        }
    }
    data
}
