{
	description = "Zenyx - A WSYWIG game engine written in rust ";
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
		utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
	};

	outputs = { self, nixpkgs, utils,rust-overlay, ... }: {
		overlays.default = final: prev: {
			zenyx = final.callPackage ./build.nix {};
		};
	}
	// 
	utils.lib.eachDefaultSystem (system:
		let pkgs = import nixpkgs {
			inherit system;
			overlays = [self.overlays.default (import rust-overlay) ];
		};
		in {
			packages = {
				inherit (pkgs) zenyx;
				default = pkgs.zenyx;
			};

			devShells.default = pkgs.mkShell {
				name = "zenyx";
				nativeBuildInputs = with pkgs; [  
        (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "cargo" "clippy" ];
          # targets = [ "arm-unknown-linux-gnueabihf" ];
        }))
					pkg-config
				];
        buildInputs = with pkgs; [
            vulkan-loader
            wayland
            libxkbcommon
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11
            xorg.libxcb
            pkg-config
        ];
			};
		}
	);
}
