{pkgs ? import <nixpkgs> {} }:


pkgs.mkShell  {

buildInputs = with pkgs; [
libxkbcommon
];


shellHook = ''
export PKG_CONFIG_PATH="${pkgs.libxkbcommon.dev}/lib/pkgconfig"
export LD_LIBRARY_PATH="${pkgs.libxkbcommon}/lib"
'';
}
