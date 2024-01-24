{ pkgs ? import <nixpkgs> { } }:

with pkgs;

let
  rustup-toolchain = rust-bin.fromRustupToolchainFile ./rustup-toolchain.toml;
in
mkShell rec {
  packages = [
    cargo-audit
    cargo-edit
    cargo-insta
    cargo-make
    cargo-outdated
    cargo-tarpaulin
    mdbook
    mdbook-linkcheck
    rustup-toolchain
    typos
  ];

  shellHook = ''
    # Add the outputs directory to PATH where local tools will be installed.
    PATH="$PATH:$out/bin"

    # If Hooked isn't installed, use cargo to install the local version of it.
    if ! [[ -x "$(command -v hooked)" ]]; then
      cargo install --path hooked-cli --root $out
    fi

    hooked install --silent
  '';
}
