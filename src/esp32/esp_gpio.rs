// Définitions des registres GPIO pour l'ESP32
const GPIO_OUT_REG: *mut u32 = 0x3FF44004 as *mut u32;
const GPIO_ENABLE_REG: *mut u32 = 0x3FF44020 as *mut u32;
const GPIO_IN_REG: *const u32 = 0x3FF4403C as *const u32;

// Fonction pour configurer un GPIO en sortie
pub fn set_gpio_output(gpio_num: u32) {
    unsafe {
        // Activer le GPIO en sortie
        *GPIO_ENABLE_REG |= 1 << gpio_num;
    }
}

// Fonction pour configurer un GPIO en entrée
pub fn set_gpio_input(gpio_num: u32) {
    unsafe {
        // Désactiver le GPIO en sortie (le met en entrée)
        *GPIO_ENABLE_REG &= !(1 << gpio_num);
    }
}

// Fonction pour écrire sur un GPIO
pub fn gpio_write(gpio_num: u32, value: bool) {
    unsafe {
        if value {
            *GPIO_OUT_REG |= 1 << gpio_num;
        } else {
            *GPIO_OUT_REG &= !(1 << gpio_num);
        }
    }
}

// Fonction pour lire l'état d'un GPIO
pub fn gpio_read(gpio_num: u32) -> bool {
    unsafe {
        (*GPIO_IN_REG & (1 << gpio_num)) != 0
    }
}