use build_print::info;
//use std::env;

use std::path::PathBuf;
use toml::Value;

fn main() {
    binary_path_from_cargo_toml();
    linker_be_nice();
    // make sure linkall.x is the last linker script (otherwise might cause problems with flip-link)
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

// fn get_env_vars() {
//     info!("Environment variables:");
//     for (key, value) in env::vars() {
//         info!("{}={}", key, value);
//     }
// }

// fn get_cargo_env_vars() {
//     info!("Environment variables:");
//     for (key, value) in env::vars() {
//         if key.starts_with("CARGO_") {
//             info!("{}={}", key, value);
//         }
//     }
// }

fn binary_path_from_cargo_toml() {
    let binary_name = std::env::var("BINARY_NAME").unwrap();
    //info!("BINARY_NAME: {}", binary_name);
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    //info!("{}", manifest_dir);
    let manifest_path = PathBuf::from(manifest_dir).join("Cargo.toml");
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
    build_print::println!("{:?}", bins.to_string());
    info!("BINARY_PATH: {}", bin_path);
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
        "cargo:rustc-link-arg=-Wl,--error-handling-script={}",
        std::env::current_exe().unwrap().display()
    );
}
