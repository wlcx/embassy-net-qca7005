{
  description = "Another cool rust disaster from samw.";

  inputs = {
    crane.url = "github:ipetkov/crane";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    crane,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };
      craneLib = crane.mkLib pkgs;
      src = craneLib.cleanCargoSource ./.;
      commonArgs = {
        inherit src;
        strictDeps = true;
        # Any extra buildInputs
        buildInputs = [];

      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      thisCrate = craneLib.buildPackage (
        commonArgs // {
          inherit cargoArtifacts;
        }
      );
    in {
      packages.default = thisCrate;
      apps.default = utils.lib.mkApp {drv = thisCrate;};

      # Provide a dev env with rust and rust-analyzer
      devShells.default = craneLib.devShell {
        # extra dev packages
        packages = [pkgs.rust-analyzer];
      };
      formatter = pkgs.nixfmt-nixfmt-rfc-style;
    });
}
