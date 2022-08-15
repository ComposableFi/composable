{ pkgs, kusama-bin, picasso-bin }:
with pkgs;
let
  builder =
    pkgs.callPackage ./network-builder.nix { };
in
{
  kusama-local-picasso-dev-karura-dev =
    builder.mk-shared-security-network {
      parachains = [
         {
          id = 2087;
          port = 31200;
          wsPort = 9989;
          count = 3;
          chain = "picasso-dev";
          bin = "${picasso-bin}/bin/composable";
        }];
      relaychain =
        {
          bin = "${kusama-bin}/bin/polkadot";
          chain = "kusama-local";
          port = 30555;
          wsPort = 9955;
          count = 4;
        };
    };
}
