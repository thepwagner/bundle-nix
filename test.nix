{ pkgs }:
let dependencies = [
  (pkgs.stdenvNoCC.mkDerivation {
    pname = "bigdecimal";
    version = "3.1.9";
    src = pkgs.fetchurl {
      url = "https://rubygems.org/downloads/bigdecimal-3.1.9.gem";
      hash = "sha256-L/x0IDFSGtacLfyBWpjkJqIwo9Iq6sGZWCanXav62Mw=";
    };
    dontUnpack = true;
    installPhase = ''
      mkdir -p $out
      cp $src $out/bigdecimal-3.1.9.gem
    '';
  })
  (pkgs.stdenvNoCC.mkDerivation {
    pname = "meow-bundler";
    version = "0.1.18";
    src = pkgs.fetchurl {
      url = "https://rubygems.org/downloads/meow-bundler-0.1.18.gem";
      hash = "sha256-v2GnfKiLut8746DvnUqq0E2yO/okYWyBLOmR6N6s6Z4=";
    };
    dontUnpack = true;
    installPhase = ''
      mkdir -p $out
      cp $src $out/meow-bundler-0.1.18.gem
    '';
  })
];
in
pkgs.symlinkJoin {
  name = "gems";
  paths = dependencies;
}
