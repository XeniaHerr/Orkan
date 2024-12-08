{pkgs ? import <nixpkgs> {} }:


pkgs.mkShell  {

buildInputs = with pkgs; [
pkg-config
cargo
libxkbcommon
fontconfig
];

env = {

  LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
    fontconfig
    libxkbcommon
  ];
};

#shellHook = ''
#export PKG_CONFIG_PATH="${pkgs.libxkbcommon.dev}/lib/pkgconfig:${pkgs.fontconfig.dev}/lib/pkgconfig"

#export LD_LIBRARY_PATH="${pkgs.libxkbcommon}/lib"



#'';
}
