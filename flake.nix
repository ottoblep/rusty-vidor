{
  description = "Embedded rust devenv";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            rust-analyzer
            probe-rs-tools
            # bossa-arduino
            arduino-cli
            cargo-generate
            cargo-binutils
            flip-link
            cargo-generate
            (rust-bin.nightly.latest.default.override {
              extensions = [ "rust-src" "llvm-tools" ];
              targets = [ "thumbv6m-none-eabi" ]; #"arm-unknown-linux-gnueabihf" ];
            })
          ];

          shellHook = ''
            export PATH="$PATH:~/.cargo/bin"
          '';
        };
      }
    );
}