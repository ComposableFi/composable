{ pkgs, chainspec, polkadot-bin, composable-bin, acala-bin, statemine-bin }:
with pkgs;
let builder = pkgs.callPackage ./network-builder.nix { };
in {
  result = builder.mk-shared-security-network {
    relaychain = {
      bin = "${polkadot-bin}/bin/polkadot";
      # NOTE: kusama-dev and kusama-local failed to connect in 10 minutes, seems need to change spec to work faster
      chain = "rococo-local";
      port = 30444;
      wsPort = 9944;
      count = 5;
      flags = [
        "--unsafe-ws-external"
        "--unsafe-rpc-external"
        "--rpc-external"
        "--ws-external"
        "--rpc-methods=Unsafe"
        "--log=xcm=trace"
      ];
    };
    parachains = [
      {
        id = 2087;
        port = 31200;
        wsPort = 9988;
        count = 3;
        chain = chainspec;
        bin = "${composable-bin}/bin/composable";
      }
      {
        id = 1000;
        port = 31220;
        wsPort = 10008;
        count = 2;
        chain = "";
        bin = "${statemine-bin}/bin/polkadot-parachain";
      }

      {
        id = 2000;
        port = 31210;
        wsPort = 9999;
        count = 1;
        chain = "karura-dev";
        bin = "${acala-bin}/bin/acala";
        flags = [
          "--unsafe-ws-external"
          "--unsafe-rpc-external"
          "--rpc-external"
          "--ws-external"
          "--rpc-cors=all"
          "--rpc-methods=Unsafe"
          "--log=xcm=trace"
          "--"
          "--execution=wasm"
        ];
      }
    ];
  };
}
