pub mod atm_gpio;
pub mod atm_uart;
pub mod atm_spi;
pub mod atm_i2c;

use atm_gpio::*;
use atm_uart::*;
use atm_spi::*;
use atm_i2c::*;




pub fn fn_ex() -> ! {
    //---------GPIO--------
    pin_mode_output(2); // Configure la broche 2 comme sortie
    pin_mode_input(3); // Configure la broche 3 comme entrée

    spi_init(); // Initialisation SPI
    

    loop {
        digital_write(2, true); // Met la broche 2 à l'état haut
        let _state = digital_read(3); // Lit l'état de la broche 3 sans utiliser la variable

        spi_send(0xA5); // Envoie d'une data exemple
        let received = spi_receive(); // Reception d'une data exemple
    }

    //---------USART-------
    usart_init(103); // 103 correspond à 9600 baud pour un CPU à 16 MHz

    // Envoi de données
    let data_to_send: u8 = 0xA5; // Exemple : 0xA5
    usart_send(data_to_send);

    // Réception de données
    let received_data = usart_receive();



    //---------SPI---------
    spi_init();

    // Envoi de données
    let data_to_send: u8 = 0x55; // Exemple : 0x55
    spi_send(data_to_send);

    // Réception de données
    let received_data = spi_receive();


    //---------I2C---------
    // Adresse du périphérique I2C (exemple : 0x50)
    let device_address = 0x50;

    // Valeur à écrire dans le registre
    let register = 0x01;
    let value_to_write = 0x42;

    // Initialisation de l'I2C (par exemple, 100 kHz pour une fréquence CPU de 16 MHz)
    i2c_init(100_000, 16_000_000);

    // Démarrer la communication
    i2c_start();

    // Écrire l'adresse du périphérique avec le bit d'écriture (0)
    i2c_write(device_address << 1 | 0);

    // Écrire le registre
    i2c_write(register);

    // Écrire la valeur dans le registre
    i2c_write(value_to_write);

    // Arrêter la communication
    i2c_stop();

    // Attendre un moment pour laisser le périphérique traiter (optionnel)
    delay_ms(10);

    // Lire une valeur depuis le même registre
    i2c_start();

    // Écrire l'adresse du périphérique avec le bit d'écriture (0)
    i2c_write(device_address << 1 | 0);

    // Écrire le registre à lire
    i2c_write(register);

    // Redémarrer la communication pour lire les données
    i2c_start();

    // Écrire l'adresse du périphérique avec le bit de lecture (1)
    i2c_write(device_address << 1 | 1);

    // Lire la donnée depuis le périphérique
    let received_value = i2c_read(false); // Lire avec un NACK pour terminer la communication

    // Arrêter la communication
    i2c_stop();

}

pub fn delay_ms(ms: u32) {
    // Implémentation simple basée sur une boucle vide
}