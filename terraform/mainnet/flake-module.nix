{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      ssh_key = "${pkgs.pass}/bin/pass github.com/ComposableFi/composable/mainnet/mantis-solver/ssh_key";
    in
    {
      packages = rec {

        # seldom to change base image for aws vmss
        node-image = nixos-generators.nixosGenerate {
          system = "x86_64-linux";
          modules = [
            ./flake/nixos-amazon.nix
          ] ++ [ ({ ... }: { amazonImage.sizeMB = 16 * 1024; }) ]
          ;
          format = "amazon";
        };

        mainnet = pkgs.writeShellApplication {
          name = "build-ts-schema";
          runtimeInputs = with pkgs; [
            opentofu
            terranix
          ];
          text = ''
            ${pkgs.opentofu} "$@"
          '';
        };
      };
    };
}
