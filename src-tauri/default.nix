{ lib
, pkg-config
, gobject-introspection
, rustPlatform
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
, makeWrapper
, frontend-build
}:
let
  pname = "deds-shell";
  version = "0.1.0";
  src = ./.;
in
rustPlatform.buildRustPackage {
  inherit version src pname;
  cargoLock.lockFile = ./Cargo.lock;

  # OPENSSL_NO_VENDOR = 1;
  # PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig:${libsoup_3.dev}/lib/pkgconfig";

  nativeBuildInputs = [
    pkg-config
    gobject-introspection
    libappindicator-gtk3
    gtk-layer-shell
    makeWrapper
  ];

  buildInputs = [
    pkg-config
    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    gtk-layer-shell
    libappindicator-gtk3
    glib
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    pango
    webkitgtk_4_1
    openssl
  ];

  postPatch = ''
    mkdir -p frontend-build
    cp -R ${frontend-build}/* frontend-build/

    substituteInPlace ./tauri.conf.json --replace '"frontendDist": "../build"' '"frontendDist": "frontend-build"'
  '';

  checkFlags = [
      # tries to mutate the parent directory
      "--skip=test_file_operation"
    ];


  meta = with lib; {
    description = "A Tauri application";
    homepage = "https://github.com/andre-brandao/your-repo";
    license = licenses.mit;
    maintainers = [ ];
    platforms = platforms.linux;
  };
}
