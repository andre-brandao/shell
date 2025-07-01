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
, mkBunDerivation
}:

let
  pname = "deds-shell";
  version = "0.1.0";
  src = ./.;

  frontend-build = (mkBunDerivation {
    src = ./.;
    bunNix = ./bun.nix;
    packageJson = ./package.json;
    buildPhase = ''
      bun run build
    '';
    installPhase = ''
      cp -r ./build $out
    '';
  });
in
rustPlatform.buildRustPackage {
  inherit version src pname;

  # sourceRoot = "${pname}/src-tauri";


  cargoLock = {
    lockFile = ./src-tauri/Cargo.lock;
    # Add any output hashes if you have git dependencies
    # outputHashes = {
    #   "some-git-dep-0.1.0" = "sha256-...";
    # };
  };
  # postUnpack = ''
  #     echo "=== Available directories after unpack ==="
  #     find . -type d -name "*tauri*" || true
  #     find . -name "Cargo.toml" || true
  #     ls -la
  #     echo "=== End directory listing ==="
  #   '';

  preBuild = ''
    cd src-tauri
  '';
  # Copy the frontend build and update tauri config
  postPatch = ''

    cp ./src-tauri/Cargo.lock ./Cargo.lock
    cp ./src-tauri/Cargo.toml ./Cargo.toml

    mkdir -p frontend-build
    cp -R ${frontend-build}/* frontend-build/


    substituteInPlace ./src-tauri/tauri.conf.json --replace '"frontendDist": "../build"' '"distDir": "frontend-build"'
  '';

  nativeBuildInputs = [
    pkg-config
    gobject-introspection
    libappindicator-gtk3
    gtk-layer-shell
    makeWrapper
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

  checkFlags = [
      # tries to mutate the parent directory
      "--skip=test_file_operation"
    ];

  # Environment variables
  OPENSSL_NO_VENDOR = 1;
  PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig:${libsoup_3.dev}/lib/pkgconfig";



  meta = with lib; {
    description = "A Tauri application"; # Replace with your app description
    homepage = "https://github.com/yourusername/your-repo"; # Replace with your repo
    license = licenses.mit; # Replace with your license
    maintainers = [ ]; # Add maintainers if needed
    platforms = platforms.linux;
  };
}
