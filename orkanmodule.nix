{pkgs,lib, config, ...}:

let cfg = config.applications.orkan;

in

{

  enable = lib.mkEnableOption "orkan";

  package = lib.mkOption {
    type = lib.types.package;
    default = pkgs.callPackage ./default.nix {};
  };


  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
  };
}
