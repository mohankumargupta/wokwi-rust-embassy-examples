use build_print::info;
//use std::env;

use std::{fs::copy, path::PathBuf};
use toml::Value;

fn main() {
    copy_wokwi();
    linker_be_nice();
    // make sure linkall.x is the last linker script (otherwise might cause problems with flip-link)
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

fn linker_be_nice() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let kind = &args[1];
        let what = &args[2];

        match kind.as_str() {
            "undefined-symbol" => match what.as_str() {
                "_defmt_timestamp" => {
                    eprintln!();
                    eprintln!("💡 `defmt` not found - make sure `defmt.x` is added as a linker script and you have included `use defmt_rtt as _;`");
                    eprintln!();
                }
                "_stack_start" => {
                    eprintln!();
                    eprintln!("💡 Is the linker script `linkall.x` missing?");
                    eprintln!();
                }
                _ => (),
            },
            // we don't have anything helpful for "missing-lib" yet
            _ => {
                std::process::exit(1);
            }
        }

        std::process::exit(0);
    }

    println!(
        "cargo:rustc-link-arg=--error-handling-script={}",
        std::env::current_exe().unwrap().display()
    );
}

fn copy_wokwi() {
    let binary_name = std::env::var("BINARY_NAME").unwrap();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    //info!("Manifest dir: {}", &manifest_dir);
    let manifest_path = PathBuf::from(&manifest_dir).join("Cargo.toml");
    //info!("{}", manifest_path.display());
    let manifest_content = std::fs::read_to_string(manifest_path).unwrap();
    //build_print::println!("{:?}", manifest_content);
    let mytoml: Value = toml::from_str(&manifest_content).unwrap();
    let bins = mytoml.get("bin").unwrap();
    let bin_path = bins
        .as_array()
        .unwrap()
        .iter()
        .find(|b| b["name"].as_str().unwrap() == binary_name)
        .and_then(|b| b["path"].as_str())
        .unwrap_or_else(|| panic!("Path not found for binary: {}", binary_name));
    //build_print::println!("{:?}", bins.to_string());
    //info!("BINARY_PATH: {}", bin_path);
    let binary_src_path = PathBuf::from(bin_path);
    let binary_path = binary_src_path.parent().unwrap();
    //info!("BINARY_PATH: {}", binary_path.display());
    let in_wokwi_toml = binary_path.join("wokwi.toml");
    let in_diagram_json = binary_path.join("diagram.json");
    // info!("Wokwi in path: {}", in_wokwi_toml.display());
    // info!("Diagram in path: {}", in_diagram_json.display());
    
    let manifest_dir = PathBuf::from(&manifest_dir);
    let in_wokwi_toml_full = manifest_dir.join(in_wokwi_toml);
    let in_diagram_json_full = manifest_dir.join(in_diagram_json);
    // info!("Wokwi in path: {}", &in_wokwi_toml_full.display());
    // info!("Diagram in path: {}", &in_diagram_json_full.display());
    let out_wokwi_toml = manifest_dir.join("wokwi.toml");
    let out_diagram_json = manifest_dir.join("diagram.json");
    // info!("Wokwi out path: {}", &out_wokwi_toml.display());
    // info!("Diagram out path: {}", &out_diagram_json.display());
    info!("Copying wokwi.toml and diagram.json");
    copy(in_diagram_json_full, out_diagram_json)
        .unwrap_or_else(|_| panic!("Failed to copy diagram.json"));
    copy(in_wokwi_toml_full, out_wokwi_toml).unwrap_or_else(|_| panic!("Failed to copy wokwi.toml"));
}
