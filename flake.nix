{
  description = "dev shell";

  inputs = {
    nixpkgs.url       = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url   = "github:numtide/flake-utils";
    rust-overlay.url  = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustVersion = "1.89.0";   

        rustChannel = pkgs.rust-bin.stable.${rustVersion}.minimal.override {
          extensions = [ "rust-src" ];
          targets = ["wasm32-wasip2" "wasm32-unknown-unknown"];
        };


        baseShell = pkgs.mkShell {
          name = "devshell-base";
          packages = with pkgs; [
            wash-cli
          ];

          buildInputs = with pkgs; [
            rustChannel
            clang
            llvmPackages.libclang
            pkg-config
          ];

          shellHook = ''
            export LIBCLANG_PATH=${pkgs.llvmPackages.libclang.lib}/lib
            export LD_LIBRARY_PATH="$LIBCLANG_PATH:${pkgs.lib.makeLibraryPath [ 
              pkgs.llvmPackages.libclang.lib 
            ]}"
            if [ "$(uname -s)" = "Darwin" ]; then
              export DYLD_LIBRARY_PATH="$LD_LIBRARY_PATH"
            fi
          '';
        };

      in {
        devShells = {
          default = baseShell;
        };
      }
    );
}

