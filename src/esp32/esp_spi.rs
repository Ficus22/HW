// SPI Registers (ESP32, SPI2)
const SPI_CMD_REG: *mut u32 = 0x3FF64000 as *mut u32; // SPI Command Register
const SPI_CTRL_REG: *mut u32 = 0x3FF64004 as *mut u32; // SPI Control Register
const SPI_USER_REG: *mut u32 = 0x3FF6401C as *mut u32; // SPI User Register
const SPI_USER1_REG: *mut u32 = 0x3FF64020 as *mut u32; // SPI User1 Register
const SPI_USER2_REG: *mut u32 = 0x3FF64024 as *mut u32; // SPI User2 Register
const SPI_MOSI_DLEN_REG: *mut u32 = 0x3FF64028 as *mut u32; // SPI MOSI Data Length Register
const SPI_MISO_DLEN_REG: *mut u32 = 0x3FF6402C as *mut u32; // SPI MISO Data Length Register
const SPI_W0_REG: *mut u32 = 0x3FF64040 as *mut u32; // SPI Data Buffer (W0)
const SPI_SLAVE_REG: *mut u32 = 0x3FF640E0 as *mut u32; // SPI Slave Control Register



// Initialisation SPI en mode maitre
pub fn spi_init() {
    unsafe {
        // Configurer le SPI en mode maître
        *SPI_USER_REG = (1 << 31) | (1 << 30); // Mode maître et clock en mode standard
        *SPI_CTRL_REG = 0; // Désactive toutes les fonctionnalités spéciales par défaut
    }
}



// Initialisation SPI en mode esclave
pub fn spi_init_slave() {
    unsafe {
        // Configurer le SPI en mode esclave
        *SPI_USER_REG = 0; // Désactiver les bits qui activent le mode maître
        *SPI_SLAVE_REG = (1 << 30) | (1 << 29); // Configurer le SPI en mode esclave
    }
}


// Envoi de données en mode maître
pub fn spi_master_send(data: u32) {
    unsafe {
        // Charger les données dans le buffer (W0)
        *SPI_W0_REG = data;

        // Configurer la longueur des données MOSI à transmettre (32 bits ici)
        *SPI_MOSI_DLEN_REG = 31; // Longueur de 32 bits

        // Lancer la transmission
        *SPI_CMD_REG = 1 << 18; // Début de la transmission SPI

        // Attendre que la transmission soit terminée
        while (*SPI_CMD_REG & (1 << 18)) != 0 {}
    }
}



// Réception de données en mode esclave
pub fn spi_slave_receive() -> u32 {
    unsafe {
        // Configurer la longueur des données MISO à recevoir (32 bits ici)
        *SPI_MISO_DLEN_REG = 31; // Longueur de 32 bits

        // Lancer la réception
        *SPI_CMD_REG = 1 << 19; // Début de la réception SPI

        // Attendre que la réception soit terminée
        while (*SPI_CMD_REG & (1 << 19)) != 0 {}

        // Lire les données reçues depuis le buffer (W0)
        *SPI_W0_REG
    }
}

