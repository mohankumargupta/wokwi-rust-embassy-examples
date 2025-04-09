set shell := ["bash", "-c"]
set windows-shell := ["powershell.exe", "-c"]

bin := "hello-world"

esp32c3:
    @echo "Building for ESP32-C3"
    $env:BINARY_NAME='{{ bin }}'; cargo build --bin {{ bin }} --release --target riscv32imc-unknown-none-elf

list:
    @just --list



# dev example chip="esp32c3":
#     $env:BINARY_NAME='{{example}}'; cargo build --bin {{ example }}

build example chip="esp32c3":
    $env:BINARY_NAME='{{example}}'; cargo build --bin {{ example }} --release

# build example chip="esp32c3":
#     $env:BINARY_NAME='{{example}}'; cargo build --bin {{ example }} --release