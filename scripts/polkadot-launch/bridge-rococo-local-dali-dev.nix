# definition of parachain
# TODO: replace with zombienet
# because it allows to specify more things and tests
# more structured and portable and officially endorsed by parity
# so with nix it is easier to build own (nix+curl+websocket)

{ pkgs, polkadot-bin, composable-bin }:
with pkgs;
let builder = pkgs.callPackage ./network-builder.nix { };
in {
  result = builder.mk-shared-security-network {
    relaychain = {
      bin = "${polkadot-bin}/bin/polkadot";
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
        "--beefy"
      ];
    };
    parachains = [
      {
        id = 2087;
        port = 31200;
        wsPort = 9988;
        count = 3;
        chain = "dali-2087";
        bin = "${composable-bin}/bin/composable";
        flags = [
          "--unsafe-ws-external"
          "--unsafe-rpc-external"
          "--rpc-external"
          "--ws-external"
          "--rpc-cors=all"
          "--rpc-methods=Unsafe"
          "--execution=wasm"
          "--wasmtime-instantiation-strategy=recreate-instance-copy-on-write"
          "--log=ibc_transfer=trace,pallet_ibc=trace"
        ];
      }
      {
        id = 2000;
        port = 33200;
        wsPort = 9188;
        count = 3;
        chain = "dali-2000";
        bin = "${composable-bin}/bin/composable";
        flags = [
          "--unsafe-ws-external"
          "--unsafe-rpc-external"
          "--rpc-external"
          "--ws-external"
          "--rpc-cors=all"
          "--rpc-methods=Unsafe"
          "--execution=wasm"
          "--wasmtime-instantiation-strategy=recreate-instance-copy-on-write"
          "--log=ibc_transfer=trace,pallet_ibc=trace"
        ];
      }
    ];
  };
}
