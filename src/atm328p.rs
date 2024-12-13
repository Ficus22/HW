mod atm_gpio;
mod atm_uart;
mod atm_spi;
mod atm_i2c;

/* FONCTION EXEMPLE
    Pour l'exemple, nous allons configurer la broche 2 comme sortie et la mettre à un état haut
    Nous allons également configurer la broche 3 comme entrée et lire son état
    */

fn fn_ex() -> ! {
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