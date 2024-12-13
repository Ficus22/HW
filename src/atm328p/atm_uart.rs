// USART Registers
const UDR0: *mut u8 = 0xC6 as *mut u8; // USART I/O Data Register
const UCSR0A: *mut u8 = 0xC0 as *mut u8; // USART Control and Status Register A
const UCSR0B: *mut u8 = 0xC1 as *mut u8; // USART Control and Status Register B
const UCSR0C: *mut u8 = 0xC2 as *mut u8; // USART Control and Status Register C
const UBRR0H: *mut u8 = 0xC5 as *mut u8; // USART Baud Rate Register High Byte
const UBRR0L: *mut u8 = 0xC4 as *mut u8; // USART Baud Rate Register Low Byte

// Initialisation de l'USART
pub fn usart_init(baud_rate: u16) {
    unsafe {
        // Configurer le baud rate
        *UBRR0H = (baud_rate >> 8) as u8; // Partie haute du baud rate
        *UBRR0L = baud_rate as u8;       // Partie basse du baud rate

        // Configurer les registres de contrôle
        *UCSR0B = (1 << 3) | (1 << 4); // Activer la transmission (TX) et la réception (RX)
        *UCSR0C = (1 << 1) | (1 << 2); // Format : 8 bits de données, 1 bit d'arrêt, pas de parité
    }
}

// Envoi de données via l'USART
pub fn usart_send(data: u8) {
    unsafe {
        // Attendre que le buffer d'envoi soit vide
        while (*UCSR0A & (1 << 5)) == 0 {}
        *UDR0 = data; // Charger les données dans le buffer
    }
}

// Réception de données via l'USART
pub fn usart_receive() -> u8 {
    unsafe {
        // Attendre que des données soient reçues
        while (*UCSR0A & (1 << 7)) == 0 {}
        *UDR0 // Lire les données reçues depuis le buffer
    }
}
