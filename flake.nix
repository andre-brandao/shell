{
  description = "Tauri development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            gobject-introspection
            cargo
            cargo-tauri # Optional, Only needed if Tauri doesn't work through the traditional way.
            nodejs # Optional, this is for if you have a js frontend
            bun
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
            gtk3
            harfbuzz
            librsvg
            libsoup_3
            pango
            webkitgtk_4_1
            openssl
          ];
          # shellHook = "";
        };
      });
}
