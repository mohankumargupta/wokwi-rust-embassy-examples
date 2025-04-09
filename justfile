set shell := ["bash", "-c"]
set windows-shell := ["powershell.exe", "-c"]


esp32c3 name="hello-world":
    @echo "Building for ESP32-C3"
    $env:BINARY_NAME='{{ name }}'; cargo build --bin {{ name }} --release --target riscv32imc-unknown-none-elf

list:
    @just --list



# dev example chip="esp32c3":
#     $env:BINARY_NAME='{{example}}'; cargo build --bin {{ example }}

# build example chip="esp32c3":
#     $env:BINARY_NAME='{{example}}'; cargo build --bin {{ example }} --release

# build example chip="esp32c3":
#     $env:BINARY_NAME='{{example}}'; cargo build --bin {{ example }} --release