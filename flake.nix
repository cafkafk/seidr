{
  description = "Hon hafði um sik hnjóskulinda, ok var þar á skjóðupungr mikill, ok varðveitti hon þar í töfr sín, þau er hon þurfti til fróðleiks at hafa.";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
    treefmt-nix,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];

        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
          clippy = toolchain;
        };

        treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
        buildInputs = with pkgs; [zlib] ++ lib.optionals stdenv.isDarwin [libiconv darwin.apple_sdk.frameworks.Security];
      in rec {
        # For `nix fmt`
        formatter = treefmtEval.config.build.wrapper;

        packages = {
          # For `nix build` `nix run`, & `nix profile install`:
          default = naersk'.buildPackage rec {
            pname = "seidr";
            version = "git";

            src = ./.;
            doCheck = true; # run `cargo test` on build

            inherit buildInputs;

            nativeBuildInputs = with pkgs; [cmake pkg-config installShellFiles];

            # buildNoDefaultFeatures = true;
            # buildFeatures = "git";

            # outputs = [ "out" "man" ];

            meta = with pkgs.lib; {
              description = "A Rust GitOps/symlinkfarm orchestrator inspired by GNU Stow";
              longDescription = ''
                A Rust GitOps/symlinkfarm orchestrator inspired by GNU Stow.
                Useful for dealing with "dotfiles", and with git support as a
                first class feature. Configuration is done through a single yaml
                file, giving it a paradigm that should bring joy to those that
                use declarative operating systems and package managers.
              '';
              homepage = "https://github.com/cafkafk/seidr";
              license = licenses.gpl3;
              mainProgram = "seidr";
              maintainers = with maintainers; [cafkafk];
            };
          };

          # Run `nix build .#check` to check code
          check = naersk'.buildPackage {
            src = ./.;
            mode = "check";
            inherit buildInputs;
          };

          # Run `nix build .#test` to run tests
          test = naersk'.buildPackage {
            src = ./.;
            mode = "test";
            inherit buildInputs;
          };

          # Run `nix build .#clippy` to lint code
          clippy = naersk'.buildPackage {
            src = ./.;
            mode = "clippy";
            inherit buildInputs;
          };
        };

        # For `nix develop`:
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [rustup toolchain just convco];
        };

        # For `nix flake check`
        checks = {
          formatting = treefmtEval.config.build.check self;
          build = packages.check;
          default = packages.default;
          test = packages.test;
          lint = packages.clippy;
        };
      }
    );
}
