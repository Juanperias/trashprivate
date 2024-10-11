{
  description = "Trash crypto flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      buildInputs = with pkgs; [
        openssl
        openssl.dev

      ];
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs;
          [ libxkbcommon pkg-config ] ++ buildInputs;
        LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
        PKG_CONFIG_PATH = "${pkgs.openssl}/lib/pkgconfig:${pkgs.lib.makeLibraryPath buildInputs}";
      };
    };
}
