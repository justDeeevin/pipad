{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [ (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml) ];
        packages = [ pkgs.probe-rs-tools ];
      };
    };
}
