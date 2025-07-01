{
  mkBunDerivation,
  ...
}:
mkBunDerivation {
  src = ../.;

  bunNix = ./bun.nix;

  packageJson = ../package.json;

  buildPhase = ''
    # echo "Custom build phase for frontend"
    # bun install
    bun run build
  '';

  installPhase = ''
    cp -r ./build $out
  '';

}
