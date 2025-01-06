# Rapport d'analyse détaillée du projet

## Arborescence du projet
```css
HW
├── src
│   ├── atm328p
│   │   ├── atm_gpio.rs
│   │   ├── atm_i2c.rs
│   │   ├── atm_spi.rs
│   │   └── atm_uart.rs
│   ├── esp32
│   │   ├── esp_gpio.rs
│   │   ├── esp_i2c.rs
│   │   ├── esp_spi.rs
│   │   ├── esp_uart.rs
│   │   ├── atm328p.rs
│   │   └── esp32.rs
│   └── main.rs
├── target
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── config.toml
├── Rapport.md
└── README.md
```

## Structure globale

### 1. **main.rs**
Il s'agit du fichier principal du programme. Il sert de point d'entrée et gère l'exécution de la logique en fonction de la plateforme cible. Il utilise des modules conditionnels, sélectionnant dynamiquement le module approprié (`esp32` ou `atmega`) en fonction de la configuration, grâce aux attributs `#[cfg(feature = ...)]`. En fonction de la plateforme active, le programme appelle une fonction principale spécifique, nommée `fn_ex`, provenant du module sélectionné. Cela permet d’adapter l'exécution du programme aux différentes configurations matérielles. De plus, le programme gère les interruptions de panique en définissant un handler personnalisé. Ce gestionnaire permet de capturer les erreurs graves, d'afficher des messages d'erreur et d'effectuer des actions spécifiques en cas de panique, garantissant ainsi un comportement robuste du système.

### 2. **esp32.rs**
 Il s'agit d'un module qui regroupe tous les sous-modules liés à l'ESP32. Il implémente des fonctions pour gérer : 
 - **GPIO** : Configuration et lecture/écriture sur des broches.
 - **UART** : Initialisation, envoi et réception de données.
 - **SPI** : Initialisation et gestion de la communication SPI.
 - **I2C** : Initialisation et communication avec des périphériques I2C.
La fonction principale `fn_ex`, quant à elle, exécute des tests sur les différents périphériques en boucle.

### 3. **atm328p.rs**
Similaire à `esp32.rs`, ce fichier est designé pour le microcontrôleur ATmega328P. Il regroupe les fonctionnalités pour les GPIO, UART, SPI et I2C, tout en tenant compte des spécificités matérielles de l'ATmega328P (par exemple, les calculs pour le baud rate ou les timings). De même, la fonction principale `fn_ex` configure et teste les périphériques.

---

## Détails techniques

### **1. main.rs**
Ce fichier est conçu pour être générique, ce qui permet de sélectionner le microcontrôleur à compiler via les fonctionnalités de `Cargo.toml` (par exemple, `esp32` ou `atmega`).
#### **Fonctionnalités clés :**
- `#[panic_handler]` : Définit un comportement personnalisé pour les situations de panique, qui se limite à une boucle infinie.
- Le `fn main()` appelle simplement la fonction `fn_ex()` du module actif, qui prend en charge les tests matériels.
#### **Points forts :**
- Gestion conditionnelle des modules.
- Simplicité et lisibilité.

---

### **2. esp32.rs**
Il définit et regroupe des sous-modules pour les périphériques de l'ESP32 : GPIO, UART, SPI, et I2C. La fonction principale `fn_ex` utilise les sous-modules pour configurer les périphériques et exécuter des tests.
#### **Fonctionnalités majeures :**
- **GPIO :**
  - Initialise des broches comme entrée ou sortie.
  - Lit et écrit des valeurs sur les GPIO.
- **UART :**
  - Initialise l'UART avec une configuration de baud rate et de fréquence d'horloge.
  - Envoie et reçoit des données.
- **SPI :**
  - Initialise l'interface SPI et envoie/reçoit des données sur un bus SPI.
- **I2C :**
  - Configure l'I2C avec une fréquence d'horloge spécifique.
  - Effectue des lectures et écritures sur un périphérique I2C donné.
#### **Points forts :**
- Utilisation structurée des périphériques via des sous-modules.
- Démonstration claire des capacités matérielles de l'ESP32.

---

### **3. atm328p.rs**
Similaire à `esp32.rs`, mais adapté aux capacités limitées de l'ATmega328P. Il définit les sous-modules pour gérer les GPIO, UART, SPI, et I2C.
#### **Fonctionnalités majeures :**
- **GPIO :**
  - Configure des broches comme entrée ou sortie.
  - Lit et écrit des valeurs numériques sur les broches.
- **UART :**
  - Configure le baud rate en fonction de la fréquence d'horloge (16 MHz par défaut).
  - Envoie et reçoit des données via l'interface UART.
- **SPI :**
  - Initialise l'interface SPI.
  - Envoie et reçoit des données sur le bus SPI.
- **I2C :**
  - Implémente les opérations de base (start, stop, write, read).
  - Lit et écrit des valeurs sur un périphérique I2C donné.
#### **Points forts :**
- Bonne gestion des limitations matérielles spécifiques à l'ATmega328P.
- Implémentation complète des interfaces matérielles principales.

---

## Comparaison entre ESP32 et ATmega328P

### 1. **ESP32 :**
 - Matériel plus avancé avec une gestion hardware directe pour les horloges, GPIO, et protocoles de communication.
 - Les appels à `Peripherals` et `ClockControl` permettent d'abstraire le matériel de manière propre.
 - Plus de flexibilité dans la gestion des périphériques.

### 2. **ATmega328P :**
 - Limité par ses caractéristiques matérielles (16 MHz, absence d'abstraction avancée comme sur l'ESP32).
 - Nécessite une gestion plus "bas-niveau" (par exemple, manipulation directe des registres pour l'I2C).

