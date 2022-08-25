{pkgs, packages, ...}:
pkgs.arion.build {
  modules = [
    ({ pkgs, ... }: {
      config.project.name = "Composable Fincance devnet";
      config.services = {
        devnet-dali = import ./services/devnet-dali.nix { inherit pkgs; inherit packages; };
      };
    })
  ];
  inherit pkgs;
}

