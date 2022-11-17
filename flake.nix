{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nobbz.url = "github:nobbz/nixos-config";

    oxalica.url = "github:oxalica/rust-overlay";

    cargo2nix.url = "github:cargo2nix/cargo2nix";
    cargo2nix.inputs.rust-overlay.follows = "oxalica";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    naersk,
    oxalica,
    nobbz,
    cargo2nix,
    ...
  }:
    flake-utils.lib.eachSystem ["x86_64-linux"] (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [oxalica.overlays.default cargo2nix.overlays.default];
      };

      rsPkgs = pkgs.rustBuilder.makePackageSet {
        rustVersion = "2022-11-05";
        rustChannel = "nightly";
        extraRustComponents = ["rustfmt" "clippy" "rust-src" "rust-analyzer"];
        packageFun = import ./Cargo.nix;
      };
    in {
      formatter = nobbz.formatter.${system};

      packages.rs-tool = (rsPkgs.workspace.rs-tool {});
      packages.default = self.packages.${system}.rs-tool;

      apps.test = {
        type = "app";
        program = "${pkgs.writeShellScript "test" "cargo test --release -- --format=terse -Z unstable-options --shuffle"}";
      };

      devShells.default = rsPkgs.workspaceShell {
        packages = builtins.attrValues {
          inherit (nobbz.packages.${system}) nil;
          inherit (cargo2nix.packages.${system}) default;
          inherit (pkgs) cargo-outdated;
        };
      };
    });
}
