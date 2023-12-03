{
  description = "hibro";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.05";
  };

  outputs = {
    self,
    nixpkgs,
    ...
  } @ inputs: let
    # system = "x86_64-linux";
    systems = [
      "aarch64-darwin"
      "aarch64-linux"
      "i686-linux"
      "x86_64-darwin"
      "x86_64-linux"
    ];
    forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
  in {
    # formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.alejandra);
    formatter = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in
      pkgs.alejandra);

    devShells = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in rec {
      default = hibro;

      hibro =
        pkgs.mkShell
        {
          buildInputs = [
            pkgs.rustc
            pkgs.cargo
            pkgs.rust-analyzer
            pkgs.rustup
            pkgs.nodejs
            pkgs.websocat

            pkgs.pkg-config
            pkgs.openssl
          ];

          shellHook = ''
            # prompt
            COLOR_RESET="$({ exists tput && tput sgr0; } 2>/dev/null || printf '\033[0m')"
            COLOR_BRED="$({ exists tput && tput bold && tput setaf 1; } 2>/dev/null || printf '\033[1;31m')"
            COLOR_BGREEN="$({ exists tput && tput bold && tput setaf 2; } 2>/dev/null || printf '\033[1;32m')"
            COLOR_BYELLOW="$({ exists tput && tput bold && tput setaf 3; } 2>/dev/null || printf '\033[1;33m')"
            COLOR_BBLUE="$({ exists tput && tput bold && tput setaf 6; } 2>/dev/null || printf '\033[1;34m')"
            COLOR_BCYAN="$({ exists tput && tput bold && tput setaf 6; } 2>/dev/null || printf '\033[1;36m')"
            gitrepo() {
              echo "$(git remote -v 2>/dev/null | grep "(fetch)" | awk -F'\t' '{print $1}')"/"$(git branch 2>/dev/null | grep -e '\* ' | sed 's/^..\(.*\)/\1/')"
            }
            # PS1=$(echo "\n$COLOR_BBLUE\$(git status -s 2> /dev/null)$COLOR_RESET\n $COLOR_BGREEN·$COLOR_RESET$COLOR_BYELLOW aws-shell $COLOR_RESET$COLOR_BRED\$(gitrepo)$COLOR_RESET$COLOR_BCYAN \$(pwd | sed "s:\$\{HOME\}:~:g")$COLOR_RESET\n · ")
            PS1=$(echo "\n $COLOR_BGREEN·$COLOR_RESET$COLOR_BYELLOW hibro-devshell $COLOR_RESET$COLOR_BRED\$(gitrepo)$COLOR_RESET$COLOR_BCYAN \$(pwd | sed "s:\$\{HOME\}:~:g")$COLOR_RESET\n · ")
          '';
        };
    });
  };
}
