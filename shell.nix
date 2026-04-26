{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    cmake
    just
    nodejs_22
    pkg-config
    postgresql
    pnpm
  ];

  LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.postgresql
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.postgresql
  ];
}
