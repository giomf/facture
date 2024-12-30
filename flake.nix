{
  description = "Dev environment for facture";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell rec {
          env = {
            RUST_BACKTRACE = 1;
          };
          buildInputs = with pkgs; [
            cargo
            typst
            rustc
            rustfmt
            diesel-cli
            sqlite
            typst
            typst-lsp
          ];
          env = {
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
          };
        };
      }
    );
}
