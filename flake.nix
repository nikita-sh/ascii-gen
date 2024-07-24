{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        naersk-lib = pkgs.callPackage naersk { };
	overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = [ 
	          # cargo 
	          # rustc 
	          # rustfmt 
	          # pre-commit 
	          # rustPackages.clippy 
	          rust-bin.beta.latest.default
	        ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}
