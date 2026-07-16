{
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";
	};

	outputs = { self, nixpkgs }@inputs:
	let
		system = "x86_64-linux";
		
		pkgs = import inputs.nixpkgs{
			inherit system;
			config.allowUnfree = true;
		};
	in 
	{
		devShells.${system}.default = pkgs.mkShell rec {
			name="Horologium-lib";
			packages = with pkgs; [
          rustc
          cargo
          gcc  
          rust-analyzer
				];
			shellHook = "tmux -L Horologium-lib new-session -A -s Horologium-lib";
			};
		};
	}
