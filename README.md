# Rust on the MCH2022 badge
This repo contains instructions and code to run Rust on the MCH2022 badge.

- [Install the toolchains in your host machine]

It also contains [instructions and recommendations on how to start a new project](#creating-your-own-project).

## Installing the toolchains
Install the Rust toolchain for ESP, see full [instructions here](https://github.com/esp-rs/rust-build#xtensa-installation)
1. Install the prerequisites
   - [Linux](https://github.com/esp-rs/rust-build#prerequisites)
   - MacOs: No prerequisites are needed
2. Clone the repository or [download the installer](https://github.com/esp-rs/rust-build#download-installer)
   - If downloading the installer, make it executable: `chmod a+x install-rust-toolchain.sh`
3. Run the following command:
   - Linux/MacOs:
        ```bash
        ./install-rust-toolchain.sh \
        --extra-crates "cargo-espflash ldproxy" \
        --clear-cache "YES" --export-file ~/export-esp.sh \
        --esp-idf-version "release/v4.4" \
        --minified-esp-idf "YES" \
        --build-target "esp32"
        ```
4. Update the environment variables as told at the end of the installation script.
5. Keep a copy of the MCH2022 webusb tools close by in the same directory as this project. 
   Clone it from: https://github.com/badgeteam/mch2022-tools

## Build
```
cargo espflash save-image ESP32 rust4mch.img 
```
Or run the `build.sh` script. 

## Flash
```
../mch2022-tools/webusb_push.py rust4mch rust4mch.img
```
Or run the `upload.sh` script

# Inspiration

## Official Badge.team documentation
The official process for the badge can be found here. 
https://www.badge.team/docs/badges/mch2022/software-development/rust/

It provides a simple 'hello world' example over serial and a more advanced example. 

### Std

Use [esp-idf-template](https://github.com/esp-rs/esp-idf-template) as starting point:
```bash
cargo generate  https://github.com/esp-rs/esp-idf-template
```
`cargo-generate` will as you a few questions, after those, you will have a "Hello, world!"
Rust binary crate for the ESP-IDF framework.

Answer the questions as follows:
 - STD support: YES
 - Release: 4.4
 - MCU: ESP32
 - Dev containers: NO 

### No-Std

Use [esp-template](https://github.com/esp-rs/esp-template) as starting point:
```bash
cargo generate  https://github.com/esp-rs/esp-template
```
`cargo-generate` will as you a few questions, after those, you will have a bare-metal
minimalist project!

## Hello world example 

This project added a minimal hello world example in the `examples` directory. 
You can run it as follows:
```
cargo build --example hello_world
cd target/xtensa-esp32-espidf/debug/examples/
esptool.py --chip esp32 elf2image hello_world
```
Reset the badge. Connect a serial program to /dev/ttyACM0 at 115k2 baud. 
After that you can navigate to the apps and run the app. 

## Rust on ESP32 STD Demo App
The best example on useing std Rust on the ESP32 can be found here. It uses a lot of peripheral and protocols. 

https://github.com/ivmarkov/rust-esp32-std-demo.git

```
git clone https://github.com/ivmarkov/rust-esp32-std-demo.git
cd rust-esp32-std-demo/
```
