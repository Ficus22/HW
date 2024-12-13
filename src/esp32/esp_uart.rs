// UART Registers (ESP32, UART0)
const UART_FIFO_REG: *mut u32 = 0x3FF40000 as *mut u32;  // FIFO Register
const UART_INT_RAW_REG: *mut u32 = 0x3FF40004 as *mut u32; // Interrupt Raw Status Register
const UART_INT_CLR_REG: *mut u32 = 0x3FF40010 as *mut u32; // Interrupt Clear Register
const UART_CLKDIV_REG: *mut u32 = 0x3FF40014 as *mut u32;  // Clock Divider Register
const UART_CONF0_REG: *mut u32 = 0x3FF40020 as *mut u32;   // Configuration Register 0
const UART_CONF1_REG: *mut u32 = 0x3FF40024 as *mut u32;   // Configuration Register 1
const UART_STATUS_REG: *mut u32 = 0x3FF4001C as *mut u32;  // Status Register

// Initialisation de l'UART
pub fn uart_init(baud_rate: u32, clock_freq: u32) {
    unsafe {
        // Configurer le baud rate
        let clk_div = clock_freq / (baud_rate * 16);
        *UART_CLKDIV_REG = clk_div;

        // Configurer la communication : 8 bits de données, pas de parité, 1 bit d'arrêt
        *UART_CONF0_REG = 0; // Valeur par défaut (8N1 : 8 bits, No parity, 1 stop bit)

        // Configurer le FIFO pour recevoir et transmettre
        *UART_CONF1_REG = (120 << 8) | 120; // Définir les seuils de FIFO (réception et transmission)

        // Effacer toutes les interruptions
        *UART_INT_CLR_REG = 0xFFFFFFFF;
    }
}

// Envoi de données via l'UART
pub fn uart_send(data: u8) {
    unsafe {
        // Attendre que le FIFO TX soit prêt
        while (*UART_STATUS_REG & (1 << 16)) != 0 {}
        *UART_FIFO_REG = data as u32; // Charger les données dans le FIFO
    }
}

// Réception de données via l'UART
pub fn uart_receive() -> u8 {
    unsafe {
        // Attendre que des données soient disponibles dans le FIFO RX
        while (*UART_STATUS_REG & (1 << 0)) == 0 {}
        (*UART_FIFO_REG & 0xFF) as u8 // Lire les données depuis le FIFO
    }
}
