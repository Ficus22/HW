mod esp_gpio;
mod esp_uart;
mod esp_spi;
mod esp_i2c;

/* 
FONCTION EXEMPLE
Pour l'exemple, nous allons configurer la broche 2 comme sortie et la mettre à un état haut
Nous allons également configurer la broche 3 comme entrée et lire son état
*/

fn fn_ex() -> ! {
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