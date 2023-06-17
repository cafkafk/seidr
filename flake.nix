{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    test-directory = {
      url = "file:///home/ces/org/src/git/gg";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, fenix, flake-utils, advisory-db, test-directory, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        inherit (pkgs) lib;

        craneLib = crane.lib.${system};

        #src = craneLib.cleanCargoSource (craneLib.path ./.);

        # When filtering sources, we want to allow assets other than .rs files
        src = lib.cleanSourceWith {
          src = ./.; # The original, unfiltered source
          filter = path: type:
            (lib.hasSuffix "\.yaml" path) ||
            # (lib.hasSuffix "\.scss" path) ||
            # Example of a folder for images, icons, etc
            #(lib.hasInfix "test" path) ||
            # Default filter from crane (allow .rs files)
            (craneLib.filterCargoSources path type)
          ;
        };

        # src = pkgs.lib.cleanSourceWith {
        #    src = craneLib.path ./.; # original, unfiltered source
        #    filter = path: type:
        #      (builtins.match ".*yaml" path != null) # include JSONC files
        #      || (craneLib.filterCargoSources path type);
        #  };

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;

          buildInputs = [
            # test-directory
            # Add additional build inputs here
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];

          # Additional environment variables can be set directly
          # MY_CUSTOM_VAR = "some value";
          RUST_BACKTRACE = 1;
        };

        craneLibLLvmTools = craneLib.overrideToolchain
          (fenix.packages.${system}.complete.withComponents [
            "cargo"
            "llvm-tools"
            "rustc"
          ]);

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        my-crate = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          # doCheck = false; # NOTE remove me
        });
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit my-crate;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          my-crate-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

          my-crate-doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
          });

          # Check formatting
          my-crate-fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Audit dependencies
          my-crate-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `my-crate` if you do not want
          # the tests to run twice
          # NOTE enable this part
          my-crate-nextest = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            partitions = 1;
            partitionType = "count";
          });
        } // lib.optionalAttrs (system == "x86_64-linux") {
          # NB: cargo-tarpaulin only supports x86_64 systems
          # Check code coverage (note: this will not upload coverage anywhere)
          # my-crate-coverage = craneLib.cargoTarpaulin (commonArgs // {
          #   inherit cargoArtifacts;
          # });
        };

        packages = {
          default = my-crate;
          my-crate-llvm-coverage = craneLibLLvmTools.cargoLlvmCov (commonArgs // {
            inherit cargoArtifacts;
          });
        };

        apps.default = flake-utils.lib.mkApp {
          drv = my-crate;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks.${system};

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

          # Extra inputs can be added here
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
          ];
        };
      });
}
