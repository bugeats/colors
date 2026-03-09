{
  inputs.systems.url = "github:nix-systems/default";
  inputs.nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0";
  outputs =
    {
      self,
      systems,
      nixpkgs,
    }:
    let
      eachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = eachSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};

          colors = pkgs.rustPlatform.buildRustPackage {
            pname = "colors";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
          };

          json = pkgs.runCommand "colors-json" { } ''
            mkdir -p $out
            ${colors}/bin/colors --json > $out/colors.json
          '';
        in
        {
          default = colors;
          inherit json;
        }
      );

      overlays.default = final: prev: {
        colors = self.packages.${final.stdenv.hostPlatform.system}.json;
      };
    };
}
