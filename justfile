set shell := ["bash", "-c"]
set windows-shell := ["powershell.exe", "-c"]

default:
    @just --list

dev example:
    cargo build --bin {{ example }}

build example:
    $env:BINARY_NAME='{{example}}'; cargo build --bin {{ example }} --release