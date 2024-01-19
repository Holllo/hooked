{ pkgs ? import <nixpkgs> { } }:

with pkgs;

let
  rustup-toolchain = rust-bin.fromRustupToolchainFile ./rustup-toolchain.toml;
in
mkShell rec {
  packages = [
    cargo-edit
    cargo-make
    mdbook
    mdbook-linkcheck
    rustup-toolchain
  ];

  shellHook = ''
    # If Hooked isn't installed, use cargo to install the local version of it.
    if ! [[ -x "$(command -v hooked)" ]]; then
      cargo install --path hooked-cli
    fi
  '';
}
