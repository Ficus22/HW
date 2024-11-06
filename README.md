# Projet HAL

## Équipe
Nous sommes **Simon ARNOLD** et **Esteban CARRASCO**, du groupe **OCC1** de l'ESILV.

## Etape 1
Nous avaons dans l'étape 1, réimplémenter en **RUST** les fonction `pinMode()`, `digitalRead` et `digitalWrite` utilent lors de la manipulation d'Arduino.
Ces dernières prennent les appélations suivantes : `pin_mode_output()`, `pin_mode_input()`, `digital_write()` et `digital_read()`.

[CORRECTION GPIO] (Don't hesitate to remove this part)
I couldn't compile ! When you build your project for the first time, I recommand you to use the ```cargo new your_project``` command.
Consider subdividing your project into separate modules. 

## Compilation du projet en no_std
Ce projet utilise Rust en mode no_std, ce qui signifie qu'il est compilé sans la bibliothèque standard (std). Cela est nécessaire pour les environnements embarqués ou les systèmes où la bibliothèque standard n'est pas disponible. De plus, le gestionnaire de panic est configuré en mode abort pour éviter l'utilisation du dépliage de la pile, qui n'est pas pris en charge en mode no_std.

## Prérequis
Assurez-vous d'avoir installé la version nightly de Rust ainsi que le composant rust-src :
rustup component add rust-src --toolchain nightly-x86_64-pc-windows-msvc

## Commande de compilation
Pour compiler le projet, utilisez la commande suivante :
cargo +nightly build -Z build-std=core,alloc --target thumbv7em-none-eabihf -Z build-std-features=panic_immediate_abort


