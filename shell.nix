{ pkgs ? import <nixpkgs> { }
, unstable ? import <unstable> { }
}:

pkgs.mkShell {
  # bindgen
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  nativeBuildInputs = [
    pkgs.clang
    pkgs.llvmPackages.libclang
    pkgs.rust-analyzer
    pkgs.rustup
  ];
}
