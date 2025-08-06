{ pkgs ? import <nixpkgs> {} }:

let
  openssl = pkgs.openssl;
in
pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.pkg-config
    openssl.dev
    openssl.out
  ];

  # This helps openssl-sys find the .so files
  OPENSSL_DIR = "${openssl.dev}";
  OPENSSL_LIB_DIR = "${openssl.out}/lib";
  OPENSSL_INCLUDE_DIR = "${openssl.dev}/include";
  PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig";
}