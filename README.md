# 🦀 Firmware Embarqué Rust - TP3 & 4 (STM32 & Embassy)

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-TP3%264--Completed-success)]()

Ce projet est un **firmware embarqué asynchrone**, développé en Rust en utilisant le framework **Embassy**. Il est conçu pour fonctionner sur une carte Nucleo-64 équipée d'un microcontrôleur **STM32L476RG** surmontée d'une carte d'extension développée par l'ENSEA. 

Ce dépôt fusionne les TPs 3 et 4, illustrant la construction complète d'un système embarqué robuste, allant de l'abstraction des registres matériels jusqu'à la gestion de tâches concurrentes sécurisées.

---

## 🚀 Fonctionnalités

Le développement est structuré en quatre étapes clés :

* **Partie 1 : Board Support Package (BSP)** – Création d'une couche d'abstraction (`bsp_ensea.rs`) pour encapsuler et centraliser la configuration matérielle des broches (Pins) de la carte.
* **Partie 2 : Drivers Périphériques** – Implémentation de pilotes de haut niveau pour interagir avec le matériel :
  * **Bargraph** à 8 LEDs.
  * **Gamepad** à 5 boutons (Haut, Bas, Gauche, Droite, Centre).
  * **Encodeur rotatif** utilisant l'interface matérielle en quadrature (QEI).
  * **Moteur pas à pas** avec contrôle de la vitesse (via Timer), de la direction et du microstepping.
* **Partie 3 : Multitâche Asynchrone** – Découpage du système en tâches indépendantes et réactives (`bargraph_task`, `stepper_update_task`, `encoder_task`, `emergency_stop_task`) orchestrées par l'exécuteur Embassy.
* **Partie 4 : Synchronisation & Affichage OLED** – Résolution des *Race Conditions* à l'aide de **Mutex** et de variables atomiques pour sécuriser le partage d'état. Intégration d'un écran **OLED SSD1306** (via le bus I2C) pour afficher la télémétrie du système en temps réel.

---

## 🛠 Installation & Utilisation

### Prérequis
* [Rust](https://www.rust-lang.org/tools/install) avec la cible croisée (cross-compilation target) installée :
  ```bash
  rustup target add thumbv7em-none-eabihf
### Lancement
Une fois les prérequis installés et votre carte branchée en USB, vous pouvez compiler et flasher directement le programme principal. 
Pour compiler et exécuter le projet globalement :
  ```bash
  cargo run --bin tp3


