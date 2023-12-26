{ pkgs ? import <nixpkgs> { } }:

let
  np = import (builtins.fetchTarball {
    url =
      "https://github.com/NixOS/nixpkgs/archive/9957cd48326fe8dbd52fdc50dd2502307f188b0d.tar.gz";
  }) { };

  myPkg = np.nodejs_20;

in pkgs.mkShell {
  buildInputs = with pkgs; [
    xorg.libxcb
    pkgconfig
    xorg.libX11
    # nodejs_20
    myPkg
  ];
}
