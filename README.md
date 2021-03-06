# Rust on the MCH2022 badge
This repo contains instructions and code to run Rust on the MCH2022 badge.
There are two approaches regarding environment:
- [Install the toolchains in your host machine](#installing-the-toolchains)
- [Use devcontainers](#devcontainers)
  - VsCode: Requires VsCode, Remote - Containers extension and Docker installed.
  - Gitpod: Requires a Gitpod account which can be created with a GitLab, GitHub or Bitbucket account.
  - GitHub Codespaces: Requires Codespaces beta.

When using devcontainers, it supports [Wokwi](https://wokwi.com/) simulation:
![Rusto onMCH2022](docs/Wokwi_simulation.png)

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
## Configure Wifi
Copy the file './src/wifi_creds_example.rs' to './src/wifi_creds.rs'
Set the Wifi credentials in the file './src/wifi_creds.rs' to the proper SSID and password.
## Build
```
cargo build
```
## Flash
We are setting `cargo espflash --monitor` as custom runner in `.cargo/config.toml`, so we can use:
```
cargo run [OPTIONS] [SERIAL] [SUBCOMMAND]
```
And it will flash the target in the SERIAL port and open a serial monitor after
flashing. We can also use `cargo-espflash` directly:
```
cargo espflash [OPTIONS] [SERIAL] [SUBCOMMAND]
```
See [Usage section](https://github.com/esp-rs/espflash/tree/master/cargo-espflash#usage)
of [cargo-espflash](https://github.com/esp-rs/espflash/tree/master/cargo-espflash) for information on arguments.

## Devcontainers
 The repository supports:
-  [Gitpod](https://gitpod.io/): [![Open ESP32 in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/github.com/jhaand/rust4mch)
-  [Vs Code Devcontainers](https://code.visualstudio.com/docs/remote/containers#_installation)
-  [GitHub Codespaces](https://docs.github.com/en/codespaces/developing-in-codespaces/creating-a-codespace)


> **Note**
>
> When using VS Code Dev Containers or GitHub Codespaces, we can pull the image
> from Dockerhub instead of builing it from the Dockerfile. In order to do so,
> please, uncomment the `image` property of `.devcontainer/devcontainer.json`
> and comment the `build` property.

> **Warning**
>
> When using GitHub Codespaces, we need to make the ports
> public, [see instructions](https://docs.github.com/en/codespaces/developing-in-codespaces/forwarding-ports-in-your-codespace#sharing-a-port), in order to flash and run
> Wokwi simulations.

### Build
-  UI approach:
    - From UI: Press `Build` on the left side of the Status Bar.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Build Task` command.
    - `Terminal`-> `Run Build Task` in the menu.
    - With `Ctrl-Shift-B` or `Cmd-Shift-B`.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build`.
- Terminal approach:
    ```
    ./build.sh  [debug | release]
    ```
    > If no argument is passed, `release` will be used as default


### Flash

- UI approach:
    - From UI: Press `Build & Flash` on the left side of the Status Bar.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build & Flash`.
- Terminal approach:
  - Manual approach :
    ```
    cargo build --release
    cd target/xtensa-esp32-espidf/release/
    ```
    (After each cargo build) Convert the elf image to binary: 
    ```
    esptool.py --chip esp32 elf2image rus4mch
    ```
    Mount an uSD card and copy the .bin file to the card. Install via the `development tools -> File browser (SD card)`
    Run via the installed apps. 
  - Using `flash.sh` script: (Not supported at this moment)

    ```
    ./flash.sh [debug | release]
    ```
    > If no argument is passed, `release` will be used as default
- Any alternative flashing method from host machine.


### Wokwi Simulation

- UI approach:

    The default test task is already set to build the project, and it can be used
    in VsCode and Gitpod:
    - From UI: Press `Build & Run Wokwi` on the left side of the Status Bar.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Test Task` command
    - With `Ctrl-Shift-,` or `Cmd-Shift-,`
        > **Note**
        >
        > This Shortcut is not available in Gitpod by default.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build & Run Wokwi`.

- Terminal approach:

    ```
    ./run-wokwi.sh [debug | release]
    ```
    > If no argument is passed, `release` will be used as default

> **Warning**
>
>  The simulation will pause if the browser tab is in the background. This may
> affect the execution, especially when debugging.

#### Debugging with Wokwi

Wokwi offers debugging with GDB.

- UI approach:
    1. Run the Wokwi Simulation in `debug` profile
    2. Go to `Run and Debug` section of the IDE (`Ctrl-Shift-D or Cmd-Shift-D`)
    3. Start Debugging by pressing the Play Button or pressing `F5`
    4. Choose the proper user:
        - `esp` when using VsCode or GitHub Codespaces
        - `gitpod` when using Gitpod
- Terminal approach:
    ```
    $HOME/.espressif/tools/xtensa-esp32-elf/esp-2021r2-patch3-8.4.0/xtensa-esp32-elf/bin/xtensa-esp32-elf-gdb target/xtensa-esp32-espidf/debug/rust4mch -ex "target remote localhost:9333"
    ```
    > **Warning**
    >
    > Be sure to build the project in debug mode

    > [Wokwi Blog: List of common GDB commands for debugging.](https://blog.wokwi.com/gdb-avr-arduino-cheatsheet/?utm_source=urish&utm_medium=blog)

# Inspiration

## Creating Your own project

Using [cargo-generate](https://github.com/cargo-generate/cargo-generate) is
recomeneded. In order to install it:
 - Append it to the `--extra-crates`: `--extra-crates "cargo-espflash ldproxy cargo-generate"`
 - Install it: `cargo install cargo-generate`

### Std

Use [esp-idf-template](https://github.com/esp-rs/esp-idf-template) as starting point:
```bash
cargo generate  https://github.com/esp-rs/esp-idf-template
```
`cargo-generate` will as you a few questions, after those, you will have a "Hello, world!"
Rust binary crate for the ESP-IDF framework.

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
../../../../tools/webusb_push.py hello_world hello_world.bin
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
