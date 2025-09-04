{
  description = "Black Hole Simulation";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
    };
  };

  outputs = inputs: let
    inherit (inputs) self nixpkgs fenix flake-utils crane;
    meta = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
  in
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [fenix.overlays.default];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      rustToolchain = pkgs.fenix.minimal.withComponents [
        "cargo"
        "clippy"
        "rust-src"
        "rustc"
        "rustfmt"
        "rust-analyzer"
      ];

      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      src = pkgs.lib.fileset.toSource {
        root = ./.;
        fileset = pkgs.lib.fileset.unions [
          (craneLib.fileset.commonCargoSources ./.)
        ];
      };

      # runtime deps
      buildInputs = with pkgs; [
        alsa-lib
        udev
        wayland
        wayland-protocols
        libxkbcommon
        vulkan-loader
        vulkan-tools
      ];
      # Build deps
      nativeBuildInputs = let
        isLinux = pkgs.lib.optionals pkgs.stdenv.isLinux;
      in
        with pkgs;
          [
            pkg-config
          ]
          ++ isLinux [
            mold
          ];

      commonArgs = {
        inherit src buildInputs nativeBuildInputs;
        strictDeps = true;
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
    in {
      packages = let
        inherit (meta) name version;
      in {
        default = craneLib.buildPackage (commonArgs
          // {
            inherit version cargoArtifacts;
            inherit (commonArgs) buildInputs nativeBuildInputs;
            doCheck = false;
            pname = name;
            RUSTFLAGS = "-C link-arg=-fuse-ld=mold -C target-cpu=native";
          });
      };

      checks = {
        fmt = craneLib.cargoFmt {
          inherit (commonArgs) src;
        };

        clippy = let
          clippyScope = "--all";
        in
          craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "${clippyScope} -- --deny warnings";
            });
      };

      devShells.  default = pkgs.mkShell {
        inherit buildInputs nativeBuildInputs;

        packages = with pkgs; [
          just
          cargo-watch
          cargo-expand
        ];

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.udev
          pkgs.alsa-lib
          pkgs.vulkan-loader
          pkgs.wayland
          pkgs.wayland-protocols
          pkgs.libxkbcommon
        ];
      };
    });
}
