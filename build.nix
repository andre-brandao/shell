{ lib
, rustPlatform
, pkg-config
, gobject-introspection
, cargo-tauri
, nodejs
, bun
, rustc
, rustfmt
, libappindicator-gtk3
, gtk-layer-shell
, at-spi2-atk
, atkmm
, cairo
, gdk-pixbuf
, glib
, gtk3
, harfbuzz
, librsvg
, libsoup_3
, pango
, webkitgtk_4_1
, openssl
}:

rustPlatform.buildRustPackage rec {
  pname = "your-tauri-app"; # Replace with your app name
  version = "0.1.0"; # Replace with your app version

  src = ./.;

  cargoLock = {
    lockFile = ./src-tauri/Cargo.lock;
  };

  nativeBuildInputs = [
    pkg-config
    gobject-introspection
    cargo-tauri
    nodejs
    bun
    rustc
    rustfmt
    libappindicator-gtk3
    gtk-layer-shell
  ];

  buildInputs = [
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

  # Tauri apps often need these environment variables
  PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig:${libsoup_3.dev}/lib/pkgconfig";

  # Build the frontend first, then the Rust backend
  buildPhase = ''
    runHook preBuild

    # Build frontend if it exists
    if [ -f "package.json" ]; then
      echo "Building frontend..."
      bun install --frozen-lockfile
      bun run build
    fi

    # Build Tauri app
    echo "Building Tauri app..."
    cargo tauri build --bundles deb,appimage

    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    mkdir -p $out/bin
    mkdir -p $out/share/applications
    mkdir -p $out/share/icons

    # Install the binary
    cp src-tauri/target/release/${pname} $out/bin/

    # Install desktop file and icon if they exist
    if [ -f "src-tauri/icons/icon.png" ]; then
      cp src-tauri/icons/icon.png $out/share/icons/${pname}.png
    fi

    # Create a basic desktop file
    cat > $out/share/applications/${pname}.desktop << EOF
[Desktop Entry]
Name=${pname}
Exec=$out/bin/${pname}
Icon=${pname}
Type=Application
Categories=Utility;
EOF

    runHook postInstall
  '';

  meta = with lib; {
    description = "A Tauri application"; # Replace with your app description
    homepage = "https://github.com/yourusername/your-repo"; # Replace with your repo
    license = licenses.mit; # Replace with your license
    maintainers = [ ]; # Add maintainers if needed
    platforms = platforms.linux;
  };
}
