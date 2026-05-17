{
  description = "nvim-oxi's development shell";

  inputs = {
    neovim-0-11 = {
      url = "github:nix-community/neovim-nightly-overlay/master";
      inputs.neovim-src.follows = "neovim-src-0-11";
    };

    neovim-0-12 = {
      url = "github:nix-community/neovim-nightly-overlay/master";
      inputs.neovim-src.follows = "neovim-src-0-12";
    };

    neovim-nightly = {
      url = "github:nix-community/neovim-nightly-overlay/master";
      inputs.neovim-src.follows = "neovim-src-nightly";
    };

    neovim-src-0-11 = {
      url = "github:neovim/neovim/v0.11.7";
      flake = false;
    };

    neovim-src-0-12 = {
      url = "github:neovim/neovim/v0.12.2";
      flake = false;
    };

    neovim-src-nightly = {
      url = "github:neovim/neovim";
      flake = false;
    };

    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    inputs:
    let
      forEachSystem =
        f:
        inputs.nixpkgs.lib.genAttrs [
          "aarch64-darwin"
          "aarch64-linux"
          "x86_64-linux"
        ] (system: f system inputs.nixpkgs.legacyPackages.${system});
    in
    {
      devShells = forEachSystem (
        system: pkgs:
        let
          mkShell =
            neovim-overlay:
            pkgs.mkShell {
              buildInputs = inputs.nixpkgs.lib.optionals pkgs.stdenv.isDarwin [
                pkgs.libiconv
              ];

              packages = [
                pkgs.gcc
                pkgs.luajit
                neovim-overlay.packages.${system}.neovim
                pkgs.pkg-config
              ];
            };
        in
        {
          default = inputs.self.devShells.${system}.neovim-0-12;
          neovim-0-11 = mkShell inputs.neovim-0-11;
          neovim-0-12 = mkShell inputs.neovim-0-12;
          neovim-nightly = mkShell inputs.neovim-nightly;
        }
      );
    };
}
