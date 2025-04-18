use crate::parser::LockfileData;

pub fn derivation(gems: LockfileData) -> String {
    let mut derivation = r#"{ pkgs }:
let dependencies = [
"#.to_string();
    for (name, gem) in gems.gems {
        let gem_fn = format!("{}-{}.gem", name, gem.version);

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
"#, name, gem.version, gem_fn, gem.checksum, gem_fn));
    }
    derivation.push_str(r#"];
in
pkgs.symlinkJoin {
  name = "gems";
  paths = dependencies;
}"#);
    derivation
}