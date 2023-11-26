{
  description = "Dev environment for facture";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          devShells.default = pkgs.mkShell
            rec {
              env = {
                RUST_BACKTRACE = 1;
              };
              buildInputs = with pkgs;[
                cargo
                typst
                rustc
                rustfmt
                diesel-cli


                # WINIT_UNIX_BACKEND=wayland
                wayland

                expat
                fontconfig
                freetype
                freetype.dev
                libGL
                libxkbcommon
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                xorg.libXrandr
              ];
              env = {
                #LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
                LD_LIBRARY_PATH = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;
                FONTCONFIG_FILE = "${pkgs.fontconfig.out}/etc/fonts/fonts.conf";
                RUST_LOG = "debug";
              };
            };
        }
      );
}
