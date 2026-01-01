{
    description = "Advent of Code 2025 Rust";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, flake-utils }:
        flake-utils.lib.eachDefaultSystem (system:
            let
                pkgs = nixpkgs.legacyPackages.${system};
            in
            {
                devShells.default = pkgs.mkShell {
                    buildInputs = with pkgs; [
                        # Rust toolchain
                        cargo
                        rustc
                        rustfmt
                        clippy
                        rust-analyzer
                    ];

                    RUST_BACKTRACE = "1";
                };
            }
        );
}
