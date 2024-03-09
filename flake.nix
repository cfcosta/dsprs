{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";

    rust-dev-tools = {
      url = "github:cfcosta/rust-dev-tools.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { nixpkgs, flake-utils, rust-dev-tools, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-dev-tools.overlays.default ];
        };

        dsprs = import ./package.nix { inherit pkgs; };

        rdt = rust-dev-tools.setup pkgs {
          name = "dsprs";
          rust = rust-dev-tools.version.fromToolchainFile ./rust-toolchain.toml;
          dependencies = with pkgs; [ ];
        };
      in {
        packages = {
          inherit dsprs;
          default = dsprs;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ rdt.devShell ];

          buildInputs = with pkgs; [ ];

          shellHook = ''
            source ${./support/bash-env.sh}
            _load_env
          '';
        };
      });
}
