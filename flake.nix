{
  description = "do";

  inputs = { nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable"; };

  outputs = { self, nixpkgs, }:
    let pkgs = nixpkgs.legacyPackages."x86_64-linux";
    in {
      # todo we should use systems later
      packages."x86_64-linux" = rec {
        todo = pkgs.callPackage ./nix/package.nix { };
        default = todo;
      };
      devShells."x86_64-linux".default = pkgs.mkShell {
        packages = with pkgs; [ rustc cargo clippy rust-analyzer rustfmt ];
      };
    };
}
