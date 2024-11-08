Voici la version mise à jour de votre README, incluant une nouvelle section **Étape 2** qui traite de l'ajout de la nouvelle cible ESP32 :

# Projet HAL

## Équipe
Nous sommes **Simon ARNOLD** et **Esteban CARRASCO**, du groupe **OCC1** de l'ESILV.

## Étape 1
Dans l'étape 1, nous avons réimplémenté en **RUST** les fonctions `pinMode()`, `digitalRead()` et `digitalWrite()` utilisées lors de la manipulation d'Arduino. Ces dernières prennent les appellations suivantes : `pin_mode_output()`, `pin_mode_input()`, `digital_write()` et `digital_read()`.

## Étape 2 : Ajout de la nouvelle cible ESP32
Pour pouvoir compiler notre projet pour l'ESP32, nous avons ajouté une nouvelle cible dans notre environnement Rust. Voici les étapes que nous avons suivies :

1. **Installation de la toolchain nécessaire** :
   Nous avons installé la version nightly de Rust avec le composant `rust-src` pour permettre la compilation en mode no_std :
   ```bash
   rustup toolchain install nightly
   rustup component add rust-src --toolchain nightly
   ```

2. **Ajout de la cible ESP32** :
   Nous avons ajouté la cible pour l'ESP32 en utilisant la commande suivante :
   ```bash
   rustup target add xtensa-esp32-none-elf
   ```

3. **Configuration du projet** :
   Dans notre fichier `Cargo.toml`, nous avons défini des features pour activer le code spécifique à l'ESP32. Cela nous permet de compiler le projet pour différentes cibles (ATmega328P et ESP32) en fonction des besoins.

4. **Compilation du projet** :
   Pour compiler le projet pour l'ESP32, nous utilisons la commande suivante :
   ```bash
   cargo +nightly build --target xtensa-esp32-none-elf --release --features esp32
   ```

## Compilation du projet en no_std
Ce projet utilise Rust en mode no_std, ce qui signifie qu'il est compilé sans la bibliothèque standard (std). Cela est nécessaire pour les environnements embarqués ou les systèmes où la bibliothèque standard n'est pas disponible. De plus, le gestionnaire de panic est configuré en mode abort pour éviter l'utilisation du dépliage de la pile, qui n'est pas pris en charge en mode no_std.

## Prérequis
Assurez-vous d'avoir installé la version nightly de Rust ainsi que le composant rust-src :
```bash
rustup component add rust-src --toolchain nightly-x86_64-pc-windows-msvc
```

## Commande de compilation
Pour compiler le projet, utilisez la commande suivante pour l'ATmega328P:
```bash
cargo +nightly build --target avr-unknown-gnu-atmega328 --release --features atmega328
```
ou pour l'ESP32 :
```bash
cargo +nightly build -Z build-std=core,alloc --target xtensa-esp32-none-elf --features esp32
```



## Problèmes rencontrés
Lors du développement de ce projet, nous avons rencontré plusieurs défis :

1. **Problèmes de compatibilité des versions** : Lors de l'ajout des dépendances dans notre fichier `Cargo.toml`, nous avons eu des conflits de versions entre certaines bibliothèques (comme `esp-backtrace` et `esp-println`). Cela a nécessité plusieurs ajustements dans les versions spécifiées pour garantir la compatibilité.

2. **Configuration des cibles** : La configuration des cibles pour l'ESP32 et l'ATmega328P a nécessité une attention particulière. Nous avons dû nous assurer que les bonnes toolchains étaient installées et que les cibles étaient correctement ajoutées à notre environnement Rust.

3. **Utilisation de no_std** : Travailler en mode no_std a présenté des défis supplémentaires, notamment l'absence de certaines fonctionnalités disponibles dans la bibliothèque standard. Cela a nécessité l'utilisation de bibliothèques spécifiques à l'embarqué et une compréhension approfondie des limitations du développement sans allocation dynamique.


## Git des modifications
```bash
git status                # Vérifier l'état du dépôt
git add .                 # Ajouter toutes les modifications
git commit -m "Message"   # Valider les modifications
git push origin main      # Pousser les modifications vers le dépôt distant
```