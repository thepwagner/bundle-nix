# bundle-nix

https://github.com/fzakaria/NpmNix but for bundler/cargo

Enable on NixOS like:

```patch
-    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
+    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  nix = {
+    package = pkgs.nix.overrideAttrs (oldAttrs: {
+      src = pkgs.fetchFromGitHub {
+        owner = "NixOS";
+        repo = "nix";
+        rev = "d904921eecbc17662fef67e8162bd3c7d1a54ce0";
+        sha256 = "yqIVbJY7HkMjwZBoji0ptLuJpXUt94uk5zs0Dogt19c=";
+      };
+    });
  }

-      experimental-features = "nix-command flakes";
+      experimental-features = "nix-command flakes ca-derivations dynamic-derivations recursive-nix";
```
