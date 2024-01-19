{ pkgs ? import <nixpkgs> { } }:

with pkgs;

let
  rustup-toolchain = rust-bin.fromRustupToolchainFile ./rustup-toolchain.toml;
in
mkShell rec {
  packages = [
    cargo-make
    mdbook
    mdbook-linkcheck
    rustup-toolchain
  ];
}
