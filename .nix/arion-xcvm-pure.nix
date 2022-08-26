{pkgs, packages, ...}:
pkgs.arion.build {
  modules = [
    ({ pkgs, ... }: {
      config.project.name = "Composable Finance XCVM devnet";
      config.services = {
        junod-testing-local = import ./services/junod.nix;
      };
    })
  ];
  inherit pkgs;
}