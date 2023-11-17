# SPDX-FileCopyrightText: 2023 Christina Sørensen
# SPDX-FileContributor: Christina Sørensen
#
# SPDX-License-Identifier: AGPL-3.0-only
{
  projectRootFile = "Cargo.toml";
  programs = {
    alejandra.enable = true; # nix
    rustfmt.enable = true; # rust
    shellcheck.enable = true; # bash/shell
    deadnix.enable = true; # find dead nix code
    taplo.enable = true; # toml
    yamlfmt.enable = true; # yaml
  };
}
