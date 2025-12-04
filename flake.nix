{
  description = "Flake configuration for my blog";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      nixpkgs,
      utils,
      rust-overlay,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        DevEnv = pkgs.symlinkJoin {
          name = "dev-env";
          paths = with pkgs; [
            # frontend
            zola

            # backend
            (rust-bin.stable.latest.default.override {
              extensions = [
                "rust-analyzer"
              ];
              targets = [
                "wasm32-unknown-unknown"
              ];
            })
            cargo-watch
            cargo-edit
            wasm-pack

            # scripts dependencies
            nushell
            just
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [ DevEnv ];
        };
      }
    );
}
