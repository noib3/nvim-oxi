{
  description = "nvim-oxi";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    neovim-nightly-overlay = {
      url = "github:nix-community/neovim-nightly-overlay/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, ... }@inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        mkPkgs = isNightly: (import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
          ] ++ nixpkgs.lib.lists.optionals isNightly [
            neovim-nightly-overlay.overlay
          ];
        });

        mkShell = isNightly: (
          let
            pkgs = mkPkgs isNightly;
            inherit (pkgs) lib stdenv;
          in
          pkgs.mkShell {
            buildInputs = lib.lists.optionals stdenv.isDarwin [ pkgs.libiconv ];

            packages = with pkgs; [
              (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
              gcc
              luajit
              neovim
              pkg-config
            ];
          }
        );
      in
      {
        devShells = {
          default = mkShell false;
          nightly = mkShell true;
        };
      }
    );
}
