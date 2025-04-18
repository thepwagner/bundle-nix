let
  pkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/2631b0b7abcea6e640ce31cd78ea58910d31e650.tar.gz") {};
  fs = pkgs.lib.fileset;
  bundlenix = pkgs.stdenv.mkDerivation {
    name = "bundlenix-build";
    src = fs.toSource {
      root = ./.;
      fileset = fs.unions [
        ./.cargo
        ./src
        ./vendor
        ./Cargo.toml
        ./Cargo.lock
      ];
    };
    buildInputs = [pkgs.cargo];
    buildPhase = ''
      cargo build
    '';
    installPhase = ''
      mkdir -p $out/bin
      find target
      cp target/debug/bundle-nix $out/bin/bundle-nix
    '';
  };
  gems = pkgs.stdenvNoCC.mkDerivation {
    name = "gems.drv";
    outputHashMode = "text";
    outputHashAlgo = "sha256";
    requiredSystemFeatures = ["recursive-nix"];

    src = fs.toSource {
      root = ./.;
      fileset = fs.unions [
        ./Gemfile
        ./Gemfile.lock
      ];
    };
    buildInputs = [pkgs.nix bundlenix];

    buildPhase = ''   
      ${bundlenix}/bin/bundle-nix Gemfile.lock > derivation.nix
    '';
    installPhase = ''
      cp $(nix-instantiate derivation.nix --arg pkgs 'import ${pkgs.path} {}') $out
    '';
  };

  ruby = pkgs.stdenvNoCC.mkDerivation {
    name = "ruby";
    buildInputs = [pkgs.ruby];
    src = fs.toSource {
      root = ./.;
      fileset = fs.unions [
        ./run.rb
      ];
    };
    installPhase = ''
      mkdir $out
      cp -r $src $out
    '';
  };
in
  builtins.outputOf(ruby).outPath "out"