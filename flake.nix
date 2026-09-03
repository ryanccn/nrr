# SPDX-FileCopyrightText: 2025 Ryan Cao <hello@ryanccn.dev>
#
# SPDX-License-Identifier: GPL-3.0-or-later

{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    ferrix.url = "github:ryanccn/ferrix";
  };

  outputs =
    { ferrix, ... }@inputs:
    ferrix.lib.mkFlake inputs {
      root = ./.;

      extraPostInstall = {
        nrxAlias = {
          default = true;
          value = "ln -s $out/bin/nr{r,x}";
        };

        enableCompletions = {
          default = true;
          value = ''
            installShellCompletion --cmd nrr \
              --bash <(echo "source <(COMPLETE=bash nrr)") \
              --zsh <(echo "source <(COMPLETE=zsh nrr)") \
              --fish <(echo "source (COMPLETE=fish nrr | psub)")

            installShellCompletion --cmd nrx \
              --bash <(echo "source <(COMPLETE=bash nrx)") \
              --zsh <(echo "source <(COMPLETE=zsh nrx)") \
              --fish <(echo "source (COMPLETE=fish nrx | psub)")
          '';
        };
      };
    };
}
