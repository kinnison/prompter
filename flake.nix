{
  description = "Daniel's shell prompter";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    (flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        prompter = pkgs.rustPlatform.buildRustPackage {
          pname = "kinnison-prompter";
          version = "git";
          src = ./.;
          cargoLock = { lockFile = ./Cargo.lock; };

          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ openssl ];
        };
      in {
        packages = {
          inherit prompter;
          default = prompter;
        };
        devShell = pkgs.callPackage ./shell.nix { };
      }));
}
