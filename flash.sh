#!/usr/bin/env bash

set -e

BUILD_MODE=""
case "$1" in
    ""|"release")
        bash build.sh
        BUILD_MODE="release"
        ;;
    "debug")
        bash build.sh debug
        BUILD_MODE="debug"
        ;;
    *)
        echo "Wrong argument. Only \"debug\"/\"release\" arguments are supported"
        exit 1;;
esac


export ESP_ELF="rust4mch"
export ESP_BOARD="esp32"
if [ "${ESP_BOARD}" == "esp32c3" ]; then
    export ESP_ARCH="riscv32imc-esp-espidf"
elif [ "${ESP_BOARD}" == "esp32s2" ]; then
    export ESP_ARCH="xtensa-esp32s2-espidf"
else
    export ESP_ARCH="xtensa-esp32-espidf"
fi

web-flash --chip ${ESP_BOARD} target/${ESP_ARCH}/${BUILD_MODE}/${ESP_ELF}