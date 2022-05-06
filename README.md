Install the Rust toolchain for ESP. 

Example code for building and running Rust code on ESP32 with std. 

https://kerkour.com/rust-on-esp32

Find the code on: 
https://github.com/skerkour/kerkour.com/tree/main/2021/rust_on_esp32

Installing the esp-rs Rust toolchain now goes via: 
https://github.com/esp-rs/rust-build

Clone the repository and run: `install-rust-toolchain.sh`

Don't forget to update the PATH and LIBCLAN_PATH variables as told at the end of the installation.

Setup the default environment for esp `rustup default esp`.

It should be possible to test the installation by running `test-rust-toolchain.sh` but it didn't work on my install. 

Then download the example mention in the rust-build README.md and run the 'Cargo first approach'.


Get example source code
```
git clone https://github.com/ivmarkov/rust-esp32-std-demo.git
cd rust-esp32-std-demo/
```

Build and flash:

`cargo espflash --target <TARGET> <SERIAL>`


