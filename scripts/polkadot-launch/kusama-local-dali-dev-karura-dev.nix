# definition of parachain
# TODO: replace with zombienet
# because it allows to specify more things and tests
# more structured and portable and officially endrosed by parity
# so with nix it is easier to build own (nix+curl+websockat)

{ pkgs, polkadot-bin, composable-bin, acala-bin }:
with pkgs;
let builder = pkgs.callPackage ./network-builder.nix { };
in {
  result = builder.mk-shared-security-network {
    relaychain = {
      bin = "${polkadot-bin}/bin/polkadot";
      # NOTE: kusama-dev and kusama-local failed to conect in 10 mintues, seems need to change spec to work faster
      chain = "rococo-dev";
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
        count = 2;
        chain = "dali-dev";
        bin = "${composable-bin}/bin/composable";
      }
      {
        id = 2000;
        port = 31210;
        wsPort = 9999;
        count = 2;
        chain = "karura-dev";
        bin = "${acala-bin}/bin/acala";
        flags = [
          "--unsafe-ws-external"
          "--unsafe-rpc-external"
          "--rpc-external"
          "--ws-external"
          "--rpc-cors=all"
          "--rpc-methods=Unsafe"
          "--force-authoring"
          "--log=xcm=trace"
          "--"
          "--execution=wasm"
        ];
      }
    ];
  };
}
