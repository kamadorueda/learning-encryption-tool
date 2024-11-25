{
  inputs = {
    fenix.url = "github:nix-community/fenix";

    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable-small";

    systems.url = "github:nix-systems/default";
  };
  outputs = inputs: let
    supportedSystems = import inputs.systems;

    forEachSystem = inputs.nixpkgs.lib.genAttrs supportedSystems;

    nixkpgsOverlayForSystem = system: (nixpkgs: _: rec {
      kamadorueda = {
        python3Packages = nixpkgs.python3Packages;

        shell = nixpkgs.mkShell {
          name = "default";
          packages = [
            nixpkgs.fenix.complete.toolchain
            nixpkgs.mprocs
          ];
        };
      };
    });

    nixpkgsForSystem = system:
      import inputs.nixpkgs {
        inherit system;
        overlays = [
          inputs.fenix.overlays.default
          (nixkpgsOverlayForSystem system)
        ];
      };

    nixpkgs = forEachSystem nixpkgsForSystem;
  in {
    devShells = forEachSystem (system: {
      default = nixpkgs.${system}.kamadorueda.shell;
    });
  };
}
