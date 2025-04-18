use std::fs;
use std::io;
use std::path::Path;
use hex;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use toml::Table;
use crate::dependency::{Dependency, Dependencies};

pub struct Crate {
    pub name: String,
    pub version: String,
    pub checksum: Option<String>,
}

impl Dependency for Crate {
    fn package_url(&self) -> packageurl::PackageUrl {
        let mut purl = packageurl::PackageUrl::new("crate", &self.name).unwrap();
        purl.with_version(&self.version);
        purl
    }
}

pub fn parse_cargo_lock(path: &Path) -> Result<Dependencies<Crate>, io::Error> {
    let mut dependencies = Dependencies::new();

    let data = fs::read_to_string(path)?;
    let value = data.parse::<Table>().unwrap();

    for pkg in value["package"].as_array().unwrap() {
        let name = pkg["name"].as_str().unwrap();
        let version = pkg["version"].as_str().unwrap();

        let checksum = match pkg.get("checksum") {
            Some(checksum) => {
                let hash_bytes = hex::decode(checksum.as_str().unwrap()).unwrap();
                let hash_b64 = STANDARD.encode(hash_bytes);
                Some(format!("sha256-{}", hash_b64))
            },
            None => continue,
        };
        dependencies.runtime.push(Crate { name: name.to_string(), version: version.to_string(), checksum: checksum });
    }

    return Ok(dependencies);
}

pub fn derive_crate_nix(dependencies: &Dependencies<Crate>) -> String {
    let mut derivation = r#"{ pkgs }:
    let dependencies = [
    "#.to_string();
        for dep in &dependencies.runtime {
            let checksum = match &dep.checksum {
                Some(checksum) => checksum,
                None => {
                    eprintln!("No checksum for crate: {}", dep.name);
                    continue;
                }
            };
    
            derivation.push_str(&format!(r#"  (pkgs.stdenvNoCC.mkDerivation {{
        pname = "{}";
        version = "{}";
        src = pkgs.fetchurl {{
          name = "{}-{}.tar.gz";
          url = "https://crates.io/api/v1/crates/{}/{}/download";
          hash = "{}";
        }};
        installPhase = ''
          mkdir -p $out/{}-{}
          cp -R * $out/{}-{}
        '';
      }})
    "#, dep.name, dep.version, dep.name, dep.version, dep.name, dep.version, checksum, dep.name, dep.version, dep.name, dep.version));
        }
        derivation.push_str(r#"];
    in
    pkgs.symlinkJoin {
      name = "crates";
      paths = dependencies;
    }"#);
        derivation
}