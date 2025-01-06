// SPI Registers
const SPCR: *mut u8 = 0x2C as *mut u8; // SPI Control Register
const SPSR: *mut u8 = 0x2D as *mut u8; // SPI Status Register
const SPDR: *mut u8 = 0x2E as *mut u8; // SPI Data Register

/// Initialisation de l'interface SPI en mode maître
pub fn spi_init_master() {
    unsafe {
        // Configurer SPCR : SPI enable, Master mode, Fclk/16
        *SPCR = (1 << 6) | (1 << 4) | (1 << 0); // SPE=1, MSTR=1, SPR0=1 (fosc/16)
    }
}

/// Initialisation de l'interface SPI en mode esclave
pub fn spi_init_slave() {
    unsafe {
        // Configurer SPCR : SPI enable, Slave mode
        *SPCR = 1 << 6; // SPE=1 (Enable SPI), MSTR=0 (Slave mode par défaut)
    }
}

/// Envoi de données via SPI (mode maître)
pub fn spi_master_send(data: u8) {
    unsafe {
        *SPDR = data; // Charger les données dans le registre SPDR
        while (*SPSR & (1 << 7)) == 0 {} // Attendre que la transmission soit terminée (SPIF = 1)
    }
}

/// Réception de données via SPI (mode esclave)
pub fn spi_slave_receive() -> u8 {
    unsafe {
        while (*SPSR & (1 << 7)) == 0 {} // Attendre que la réception soit terminée (SPIF = 1)
        *SPDR // Lire les données reçues depuis le registre SPDR
    }
}

/// Échange de données (full-duplex)
pub fn spi_transfer(data: u8) -> u8 {
    unsafe {
        *SPDR = data; // Charger les données dans le registre SPDR
        while (*SPSR & (1 << 7)) == 0 {} // Attendre que la transmission/réception soit terminée
        *SPDR // Retourner les données reçues
    }
}
