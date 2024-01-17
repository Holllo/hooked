{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  packages = [ cargo-make mdbook ];
}
