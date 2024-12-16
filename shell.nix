let
  pkgs = import <nixpkgs> {};
in
  pkgs.mkShell {
    buildInputs = with pkgs; [ rustc rustfmt rust-analyzer ];
  }
