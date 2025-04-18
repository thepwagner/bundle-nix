use std::path::Path;

mod dependency;
mod rubygem;
mod rustcrate;

use crate::dependency::{Dependency, ParseFn, DerivationFn};
use crate::rubygem::{parse_gemfile_lock, derive_gem_nix};
use crate::rustcrate::{parse_cargo_lock, derive_crate_nix};

// Generic function to process dependencies
fn process_dependencies<T: Dependency>(file_path: &Path, parser: ParseFn<T>, derivation: DerivationFn<T>) -> Result<(), Box<dyn std::error::Error>> {
    let dependencies = parser(file_path)?;

    // for dependency in &dependencies.runtime {
    //     println!("{}", dependency.package_url());
    // }
    // println!("---");

    let nix = derivation(&dependencies);
    println!("{}", nix);

    Ok(())
}

fn main() {
    let file_name = match std::env::args().nth(1) {
        Some(file_name) => file_name,
        None => {
            eprintln!("Error: No file name provided.");
            std::process::exit(1);
        }
    };

    let fn_path = Path::new(&file_name);
    if !fn_path.exists() {
        eprintln!("Error: {} not found in the current directory.", fn_path.display());
        eprintln!("Please place a Gemfile.lock file in the same directory as the executable or provide the correct path.");
        std::process::exit(1);
    }

    // Determine the file type and parse accordingly
    let result = match fn_path.file_name().unwrap().to_str().unwrap() {
        "Gemfile.lock" => {
            process_dependencies(fn_path, parse_gemfile_lock, derive_gem_nix)
        }
        "Cargo.lock" => {
            process_dependencies(fn_path, parse_cargo_lock, derive_crate_nix)
        }
        _ => {
            eprintln!("Error: {} is not a supported lockfile format.", fn_path.display());
            std::process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error processing dependencies: {}", e);
        std::process::exit(1);
    }
}
