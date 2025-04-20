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

  gemsDrv = pkgs.stdenvNoCC.mkDerivation {
    name = "gems.drv";
    __contentAddressed = true;
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

  rubyVendor = builtins.outputOf(gemsDrv).outPath "out";

  rubyApp = pkgs.stdenvNoCC.mkDerivation {
    name = "ruby";
    src = fs.toSource {
      root = ./.;
      fileset = fs.unions [
        ./run.rb
      ];
    };
    buildPhase = ''
      mkdir $out
      ln -s $src/* $out
    '';
    # TODO: this does not work and I'm going to give up on why not
    installPhase = ''
      mkdir -p $out/vendor/cache
      ln -s ${builtins.outputOf(gemsDrv).outPath "out"} $out/vendor/cache
    '';
  };

  cratesDrv = pkgs.stdenvNoCC.mkDerivation {
    name = "crates.drv";
    __contentAddressed = true;
    outputHashMode = "text";
    outputHashAlgo = "sha256";
    requiredSystemFeatures = ["recursive-nix"];

    src = fs.toSource {
      root = ./.;
      fileset = fs.unions [
        ./Cargo.toml
        ./Cargo.lock
      ];
    };
    buildInputs = [pkgs.nix bundlenix];

    buildPhase = ''
      ${bundlenix}/bin/bundle-nix Cargo.lock > derivation.nix
    '';
    installPhase = ''
      cp $(nix-instantiate derivation.nix --arg pkgs 'import ${pkgs.path} {}') $out
    '';
  };
  cargoVendor = builtins.outputOf(cratesDrv).outPath "out";
in
  {
    inherit bundlenix;
    inherit gemsDrv rubyVendor rubyApp;
    inherit cratesDrv cargoVendor;
  }
