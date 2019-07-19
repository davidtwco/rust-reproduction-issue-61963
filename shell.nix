let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "reproduction-issue-61963";
    buildInputs = [ (nixpkgs.rustChannelOf { date = "2019-07-19"; channel = "nightly"; }).rust ];
  }
