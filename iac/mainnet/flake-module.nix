{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      ssh_key  = "${pkgs.pass}/bin/pass github.com/ComposableFi/composable/mainnet/mantis-solver/ssh_key";
    in
    {
      packages = rec {


        iac = pkgs.writeShellApplication {
          name = "build-ts-schema";
          runtimeInputs = with pkgs; [  
            opentofu
          ];
          text = ''
          ${pkgs.opentofu} "$@"
          '';
        };
      };
    };
}
