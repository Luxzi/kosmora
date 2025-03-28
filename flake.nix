{
  description = "Kosmora - A space efficient virtual filesystem written in RUst.";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
    alejandra.url = "github:kamadorueda/alejandra/3.1.0";
    alejandra.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    rust-overlay,
    alejandra,
    ...
  }:
    {
      overlays.default = final: prev: {
      };
    }
    // utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            self.overlays.default
            (import rust-overlay)
          ];
        };
        buildInputs = with pkgs; [
          pkg-config
        ];
      in {
        devShells.default = pkgs.mkShell {
          name = "kosmora";
          nativeBuildInputs = with pkgs; [
            rust-bin.stable.latest.default
            pkg-config
          ];
          buildInputs = buildInputs;
          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
            echo "Rust version: $(rustc --version)";
          '';
        };

        formatter = alejandra.packages.${system}.default;
      }
    );
}
