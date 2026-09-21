{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";
    crane.url = "github:ipetkov/crane";
  };
  outputs = { self, nixpkgs, crane }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true;
      };
      craneLib = crane.mkLib pkgs;

      sqlFilter = path: _type: builtins.match ".*\\.sql$" path != null;
      filter = path: type: (sqlFilter path type) || (craneLib.filterCargoSources path type);
      src = pkgs.lib.cleanSourceWith { src = ./.; inherit filter; };

      horologiumCliToml = fromTOML (builtins.readFile ./horologium-cli/Cargo.toml);
      horologiumSvcToml = fromTOML (builtins.readFile ./horologium-svc/Cargo.toml);
    in
      {
      devShells.${system}.default = pkgs.mkShell {
        name = "horologium";
        packages = with pkgs; [
          rustc
          cargo
          gcc
          rust-analyzer
        ];
        shellHook = "tmux -L horologium new-session -A -s horologium";
      };

      packages.${system} = {
        horologium-cli = craneLib.buildPackage {
          inherit src;
          cargoToml = ./horologium-cli/Cargo.toml;
          pname = horologiumCliToml.package.name;
          version = horologiumCliToml.package.version;
          cargoExtraArgs = "-p ${horologiumCliToml.package.name}";
        };

        horologium-svc = craneLib.buildPackage {
          inherit src;
          cargoToml = ./horologium-svc/Cargo.toml;
          pname = horologiumSvcToml.package.name;
          version = horologiumSvcToml.package.version;
          cargoExtraArgs = "-p ${horologiumSvcToml.package.name}";
        };

        default = self.packages.${system}.horologium-cli;
      };
      nixosModules.default = { pkgs, ... }: {
        systemd.user.services.horologium-svc = {
          description = "Horologium service";
          wantedBy = [ "default.target" ];
          serviceConfig = {
            ExecStart = "${self.packages.${pkgs.stdenv.hostPlatform.system}.horologium-svc}/bin/horologium-svc";
            Restart = "always";
          };
        };
      };
    };
}
