pub mod esp_gpio;
pub mod esp_uart;
pub mod esp_spi;
pub mod esp_i2c;

use esp_gpio::*;
use esp_uart::*;
use esp_spi::*;
use esp_i2c::*;


pub fn fn_ex() -> ! {
    //---------GPIO--------
    let peripherals = Peripherals::take().unwrap(); // initialisation de l'accès hardware de l'esp32
    let system = peripherals.DPORT.split(); // séparation des DPORT
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze(); // initialisation de la clock

    // Définition des numéros de GPIO pour l'ESP32
    const GPIO_OUTPUT: u32 = 2;  // Utilisation du GPIO2 comme sortie
    const GPIO_INPUT: u32 = 3;   // Utilisation du GPIO3 comme entrée

    // Configuration des GPIO
    set_gpio_output(GPIO_OUTPUT);  // Configure le GPIO2 comme sortie
    set_gpio_input(GPIO_INPUT);    // Configure le GPIO3 comme entrée

    loop {
        gpio_write(GPIO_OUTPUT, true);  // Met le GPIO2 à l'état haut
        let _state = gpio_read(GPIO_INPUT);  // Lit l'état du GPIO3 sans utiliser la variable
    }


    //---------USART-------
    
    uart_init(9600, 80_000_000);// Initialisation de l'UART avec un baud rate de 9600 et un clock de 80 MHz

    // Données à envoyer
    let data_to_send: u8 = 0x7E; // Exemple : 0x7E
    uart_send(data_to_send);

    // Réception de données
    let received_data = uart_receive();

    
    //---------SPI---------

    spi_init_master();

    // Envoi d'un message en mode maître
    let data_to_send: u32 = 0x1234ABCD;
    spi_master_send(data_to_send);

    // Initialisation SPI en mode esclave
    spi_init_slave();

    // Réception de données en mode esclave
    let received_data = spi_slave_receive();


    //---------I2C---------
    // Adresse du périphérique I2C (exemple : 0x50)
    let device_address = 0x50;

    // Valeur à écrire dans le registre
    let register = 0x01;
    let value_to_write = 0x42;

    // Initialisation de l'I2C (par exemple, 100 kHz pour une fréquence CPU de 80 MHz)
    i2c_init(100_000, 80_000_000);

    // Écrire une valeur dans le registre du périphérique
    let write_data = [register, value_to_write];
    i2c_send(&write_data, device_address);


    // Lire une valeur depuis le même registre
    // D'abord envoyer le registre à lire
    let register_to_read = [register];
    i2c_send(&register_to_read, device_address);

    // Ensuite, lire la donnée depuis le périphérique
    let received_data = i2c_receive(1, device_address);

}

pub fn delay_ms(ms: u32) {
    // Implémentation simple basée sur une boucle vide
}
