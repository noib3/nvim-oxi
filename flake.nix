{
  description = "nvim-oxi";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils = {
      url = "github:numtide/flake-utils";
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

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        inherit (nixpkgs.lib) lists;

        mkPkgs = isNightly: (import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
          ] ++ lists.optionals isNightly [
            neovim-nightly-overlay.overlay
          ];
        });

        mkShell = { nightly }: (
          let
            pkgs = mkPkgs nightly;
            inherit (pkgs) lib stdenv;
          in
          pkgs.mkShell {
            buildInputs = lists.optionals stdenv.isDarwin [ pkgs.libiconv ];

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
          default = mkShell { nightly = false; };
          nightly = mkShell { nightly = true; };
        };
      }
    );
}
