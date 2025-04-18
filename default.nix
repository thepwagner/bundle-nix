let
  pkgs =
    import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/2631b0b7abcea6e640ce31cd78ea58910d31e650.tar.gz") {
    };
  fs = pkgs.lib.fileset;
in
  with pkgs;
    builtins.outputOf
    (stdenv.mkDerivation {
      name = "gems.drv";
      outputHashMode = "text";
      outputHashAlgo = "sha256";
      requiredSystemFeatures = ["recursive-nix"];

      src = fs.toSource {
        root = ./.;
        fileset = fs.unions [
          ./.cargo
          ./src
          ./vendor
          ./Cargo.toml
          ./Cargo.lock

          ./Gemfile
          ./Gemfile.lock
        ];
      };

      buildInputs = [nix cargo];

      buildPhase = ''   
        cargo run Gemfile.lock > derivation.nix
      '';

      installPhase = ''
        cp $(nix-instantiate derivation.nix --arg pkgs 'import ${pkgs.path} {}') $out
      '';
    }).outPath "out"