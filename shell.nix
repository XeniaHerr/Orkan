{pkgs ? import <nixpkgs> {} }:


pkgs.mkShell  {

buildInputs = with pkgs; [
pkg-config
cargo
libxkbcommon
fontconfig
rust-analyzer
];

env = {

  LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
    fontconfig
    libxkbcommon
  ];
};

}
