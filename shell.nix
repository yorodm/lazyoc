{ pkgs ? import <nixpkgs> { } }:

let
  unstable = import (builtins.fetchTarball
    "https://github.com/nixos/nixpkgs/tarball/nixpkgs-unstable") { };
in pkgs.mkShell {
  buildInputs =
    [ pkgs.rustc pkgs.cargo unstable.cargo-generate pkgs.rust-analyzer ];
}
