{ pkgs ? import ./pkgs.nix {}, ci ? false }:

with pkgs;
mkShell {
  nativeBuildInputs = [
    shellcheck
    gitAndTools.gh
    rustc
    clippy
    rustfmt
    cargo
    cmake
    # Rust bindgen hook (necessary to build boring)
    rustPlatform.bindgenHook
  ];
  # Don't set rpath for native addons
  NIX_DONT_SET_RPATH = true;
  NIX_NO_SELF_RPATH = true;
  RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = "${stdenv.cc.cc.lib}/lib";
}
