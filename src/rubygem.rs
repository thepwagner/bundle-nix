use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use hex;
use base64::{engine::general_purpose::STANDARD, Engine as _};

use crate::dependency::{Dependency, Dependencies};

#[derive(Debug)]
pub struct RubyGem {
    pub name: String,
    pub version: String,
    pub checksum: Option<String>,
}

impl Dependency for RubyGem {
    fn package_url(&self) -> packageurl::PackageUrl {
        let mut purl = packageurl::PackageUrl::new("gem", &self.name).unwrap();
        purl.with_version(&self.version);
        purl
    }
}

pub fn parse_gemfile_lock(path: &Path) -> Result<Dependencies<RubyGem>, io::Error> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut dependencies = Dependencies::new();
    let mut in_checksums = false;

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    for line in &lines {
        let trimmed = line.trim();
        if trimmed == "CHECKSUMS" {
            in_checksums = true;
            continue;
        }
        if !in_checksums {
            continue;
        }
        if trimmed.is_empty() {
            in_checksums = false;
            continue;
        }

        if let Some(name_start) = trimmed.find(" (") {
            let name = trimmed[..name_start].trim();

            if let Some(version_start) = trimmed[name_start + 2..].find(") ") {
                let version = trimmed[name_start + 2..(name_start + 2+version_start)].trim();
                let digest = trimmed[(name_start+4+version_start)..].trim();

                let (algo, hash) = digest.split_once("=").unwrap();
                if algo != "sha256" {
                    eprintln!("Unsupported hash algorithm: {}", algo);
                    continue;
                }

                let hash_bytes = hex::decode(hash).unwrap();
                let hash_b64 = STANDARD.encode(hash_bytes);
                dependencies.runtime.push(RubyGem {
                    name: name.to_string(),
                    version: version.to_string(),
                    checksum: Some(format!("sha256-{}", hash_b64)),
                });
            }
        }
    }

    return Ok(dependencies);
}

pub fn derive_gem_nix(dependencies: &Dependencies<RubyGem>) -> String {
    let mut derivation = r#"{ pkgs }:
let dependencies = [
"#.to_string();
    for gem in &dependencies.runtime {
        let gem_fn = format!("{}-{}.gem", gem.name, gem.version);

        let checksum = match &gem.checksum {
            Some(checksum) => checksum,
            None => {
                eprintln!("No checksum for gem: {}", gem.name);
                continue;
            }
        };

        derivation.push_str(&format!(r#"  (pkgs.stdenvNoCC.mkDerivation {{
    pname = "{}";
    version = "{}";
    src = pkgs.fetchurl {{
      url = "https://rubygems.org/downloads/{}";
      hash = "{}";
    }};
    dontUnpack = true;
    installPhase = ''
      mkdir -p $out
      cp $src $out/{}
    '';
  }})
"#, gem.name, gem.version, gem_fn, checksum, gem_fn));
    }
    derivation.push_str(r#"];
in
pkgs.symlinkJoin {
  name = "gems";
  paths = dependencies;
}"#);
    derivation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rubygem_package_url() {
        let gem = RubyGem {
            name: "rails".to_string(),
            version: "8.0.0".to_string(),
            checksum: None,
        };

        let purl = gem.package_url();
        assert_eq!(purl.to_string(), "pkg:gem/rails@8.0.0");
    }
}