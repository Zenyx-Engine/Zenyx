{ lib, rustPlatform, nix-gitignore, bash, makeWrapper, dav1d, pkg-config }: 
let 
	version = (builtins.fromTOML (builtins.readFile ./engine/Cargo.toml)).package.version;
	src = nix-gitignore.gitignoreSource [] ./.;
in 
rustPlatform.buildRustPackage rec {
	pname = "zenyx";
	inherit src version;
	cargoLock.lockFile = ./Cargo.lock;
	nativeBuildInputs = [
		makeWrapper
		pkg-config
	];
	buildInputs = [
		bash
		dav1d
	];
	doCheck = false;

	fixupPhase = ''
		wrapProgram $out/bin/${pname} --set PATH ${bash}/bin:\$PATH
	'';

	meta = {
		description = "Test";
		license = lib.licenses.mit;
		platforms = lib.platforms.linux;
		mainProgram = "zenyx";
	};
}
