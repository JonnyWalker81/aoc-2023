let
  pkgs = import <nixpkgs> { };
  # choose the ocaml version you want to use
  # ocamlPackages = pkgs.ocamlPackages;
  ocamlPackages = pkgs.ocaml-ng.ocamlPackages_4_13;
  # jingoo = import (builtins.fetchGit {
  #   url = "https://github.com/tategakibunko/jingoo";
  #   rev = "60922e161db7a1320469071988d628dfd9b51eff";
  # });
  # ocamlPackages.jingoo = pkgs.fetchFromGitHub {
  #   owner = "tategakibunko";
  #   repo = "jingoo";
  #   rev = "60922e161db7a1320469071988d628dfd9b51eff";
  #   sha256 = "7WYwrcogOude46AaUn0cSFfM5wmq0n9iORilcsOrHiM=";
  # };
in pkgs.mkShell {
  # build tools
  nativeBuildInputs = with ocamlPackages; [
    pkgs.inotify-tools
    pkgs.opam
    pkgs.nodejs
    ocaml
    findlib
    dune_2
    ocaml-lsp
    pkgs.ocamlformat
    merlin
    ocp-indent
    ocamlPackages.janeStreet.base
    ocamlPackages.janeStreet.async
    ocamlPackages.janeStreet.async_unix
    ocamlPackages.janeStreet.core_unix
    ocamlPackages.janeStreet.ppx_let
    ocamlPackages.cmdliner
    ocamlPackages.ppx_deriving
    ocamlPackages.odoc
    ocamlPackages.yojson
    ocamlPackages.utop
    ocamlPackages.cohttp
    ocamlPackages.cohttp-async
    ocamlPackages.alcotest
    ocamlPackages.fmt
    ocamlPackages.jingoo
    ocamlPackages.omd
    ocamlPackages.ocaml-lsp
    # jingoo
  ];
  # dependencies
  buildInputs = with ocamlPackages; [ ocamlgraph pkgs.ocamlformat pkgs.nodejs pkgs.yarn ];
}
