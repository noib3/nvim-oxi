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
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            # neovim-nightly-overlay.overlay
          ];
        };

        inherit (pkgs) lib stdenv;
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = lib.lists.optionals stdenv.isDarwin [ pkgs.libiconv ];

          packages = with pkgs; [
            gcc
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
            neovim
          ];
        };
      }
    );
}
