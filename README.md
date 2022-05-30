# Run Rust on the MCH2022 badge

## Installing the toolchain 
Install the Rust toolchain for ESP. 

Example code for building and running Rust code on ESP32 with std. 

https://kerkour.com/rust-on-esp32

Find the code on: 
https://github.com/skerkour/kerkour.com/tree/main/2021/rust_on_esp32

Installing the esp-rs Rust toolchain now goes via: 
https://github.com/esp-rs/rust-build

Clone the repository and run: `install-rust-toolchain.sh`

Don't forget to update the PATH and LIBCLAN_PATH variables as told at the end of the installation.

Setup the default environment for esp in this directory `rustup override set esp`.

Then download the example mention in the rust-build README.md and run the 'Cargo first approach'.

## Configure Wifi
Copy the file './src/wifi_creds_example.rs' to './src/wifi_creds.rs'
Set the Wifi credentials in the file './src/wifi_creds.rs' to the proper SSID and password. 

## Build and flash:

`cargo build`

`espflash /dev/ttyUSB0 target/xtensa-esp32-espidf/debug/rust-esp32-std-demo`

# Creating Your own project

Create a new Rust project with `cargo init` and change directory to the new project directory. \
Set the default target with `rustup override set esp` \
Copy the sdkconfig*, build.rs, partitions.csv and Cargo.toml from this project to your own project. 
Create a directory .cargo and copy the `.cargo/config.toml` from this project to your new .cargo directory. 
Add the 'sdkconfig.default*' and 'build.rs' files from the example project. 

# Inspiration

## Get example source code
```
git clone https://github.com/ivmarkov/rust-esp32-std-demo.git
cd rust-esp32-std-demo/
```

