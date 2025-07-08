{
  description = "Deds Shell - A Svelte and Tauri Linux Shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    bun2nix.url = "github:baileyluTCD/bun2nix";
    bun2nix.inputs.nixpkgs.follows = "nixpkgs";
  };
  nixConfig = {
      extra-substituters = [
        "https://cache.nixos.org"
        "https://cache.garnix.io"
      ];
      extra-trusted-public-keys = [
        "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
        "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="
      ];
  };

  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        frontend-build = pkgs.callPackage ./nix/frontend.nix {  inherit (inputs.bun2nix.lib.${system}) mkBunDerivation;};
      in
      {
        packages.default = pkgs.callPackage ./src-tauri/default.nix {inherit frontend-build;};
        packages.frontend= frontend-build;

        devShells.default = pkgs.mkShell {
          GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules/";
          nativeBuildInputs = with pkgs; [
            pkg-config
            gobject-introspection
            cargo
            cargo-tauri # Optional, Only needed if Tauri doesn't work through the traditional way.
            nodejs # Optional, this is for if you have a js frontend
            bun
            inputs.bun2nix.packages.${system}.default
            rustc
            rust-analyzer
            cargo-tauri
            rustfmt
            libappindicator-gtk3
            gtk-layer-shell
          ];
          buildInputs = with pkgs; [
            at-spi2-atk
            atkmm
            cairo
            gdk-pixbuf
            glib
            glib-networking
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            webkitgtk_4_1
            openssl
          ];
          shellHook = ''
            echo "Welcome to the Deds Shell development environment!"
            echo "You can run 'bun run tauri dev' to start the development."
          '';
        };
      });
}
