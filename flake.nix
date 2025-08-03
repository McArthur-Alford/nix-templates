{
  description = "A collection of templates";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
    }:
    {
      nixosModules = {
        default =
          {
            config,
            pkgs,
            lib,
            ...
          }:
          {
            nix.registry.devtemplates = {
              to = {
                owner = "McArthur-Alford";
                repo = "nix-templates";
                type = "github";
              };
              from = {
                id = "devtemplates";
                type = "indirect";
              };
            };
          };
      };
      templates = {
        default = {
          path = ./default;
          description = "Default";
          welcomeText = ''
            # A empty, default template
            ...to modify to your hearts content
          '';
        };
        rust = {
          path = ./rust;
          description = "Rust";
          welcomeText = ''
            # A simple Rust/Cargo template
            Provides access to rust and cargo. Obviously.
          '';
        };
        rust_minifb = {
          path = ./rust_minifb;
          description = "Rust minifb scaffolding for quick & dirty visualisation";
          welcomeText = ''
            # A simple rust minifb template
            Provides scaffolding for quick & dirty visualisation with a framebuffer and glam
          '';
        };
        c = {
          path = ./c;
          description = "C";
          welcomeText = ''
            # A simple C template
            Provides GCC and clang
          '';
        };
        # haskell = {};
        # java = {};
        python311 = {
          path = ./python311;
          description = "Python311";
          welcomeText = ''
            # A simple python 3.11 template
          '';
        };
        torchrocm = {
          path = ./torchrocm;
          description = "Python311 With Pytorch/ROCM";
          welcomeText = ''
            # A convoluted python 3.11 template with torch/ROCM support
          '';
        };
        latex = {
          path = ./latex;
          description = "LaTeX Devshell";
          welcomeText = ''
            # Welcome to the magical world of LaTeX!
          '';
        };
        elixir = {
          path = ./elixir;
          description = "Elixir Devshell";
          welcomeText = ''
            # Welcome to the alchemy laboratory
          '';
        };
        # c = {};
      };
    };
}
