{lib, rustPlatform, fetchFromGitHub, cargo, rustc, libxkbcommon, fontconfig, ...}:

rustPlatform.buildRustPackage rec {
  pname = "orkan";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "XeniaHerr";
    repo = "orkan";
    rev = version;

  };

  nativeBuildInputs = [
    cargo
    rustc
    libxkbcommon
    fontconfig
  ];

  cargoHash = lib.fakeHash;

  meta = {
    description = "Wayland Programm launcher and Selector similar to dmenu";
    license = lib.licences.mit;
    maintainers = [ ];

  };
  }
