{
  description = "nvim-oxi";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nixpkgs-neovim-0-11.url =
      "github:NixOS/nixpkgs/832efc09b4caf6b4569fbf9dc01bec3082a00611";

    flake-utils = {
      url = "github:numtide/flake-utils";
    };

    neovim-nightly-overlay = {
      url = "github:nix-community/neovim-nightly-overlay/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    with inputs;
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        inherit (nixpkgs.lib) lists;

        mkPkgs =
          { nixpkgs, nightly }:
          import nixpkgs {
            inherit system;
            overlays = lists.optionals nightly [
              neovim-nightly-overlay.overlays.default
            ];
          };

        mkShell =
          { nixpkgs, nightly }:
          (
            let
              pkgs = mkPkgs { inherit nixpkgs nightly; };
            in
            pkgs.mkShell {
              buildInputs = lists.optionals pkgs.stdenv.isDarwin [ pkgs.libiconv ];

              packages = with pkgs; [
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
          default = inputs.self.devShells.${system}.neovim-0-12;
          neovim-0-11 = mkShell {
            nixpkgs = nixpkgs-neovim-0-11;
            nightly = false;
          };
          neovim-0-12 = mkShell {
            inherit nixpkgs;
            nightly = false;
          };
          nightly = mkShell {
            inherit nixpkgs;
            nightly = true;
          };
        };
      }
    );
}
