{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    apps = let
      rust_log = ''
        RUST_LOG=info,runtime=info,parachain=trace,cumulus-collator=trace,aura=debug,xcm=trace,pallet_ibc=debug,hyperspace=trace,hyperspace_parachain=trace,ics=trace,ics::routing=trace,ics::channel=trace,parachain::network-bridge-rx=debug,parachain::availability-store=info,parachain::approval-distribution=info,parachain::approval-voting=info,parachain::bitfield-distribution=debug,runtime::system=info,parachain::chain-api=debug,ics::routing=info,orml_xtokens=debug
        export RUST_LOG
      '';
    in {
      composable-follow-archive = {
        type = "app";
        program = pkgs.writeShellApplication {
          name = "composable-follow";
          runtimeInputs = [ self'.packages.composable-node ];
          text = ''

            ${rust_log}

            # with polkadot 0.9.39 can enable warp, so it will start instantly, btw fast modes do not work because cannot restart on disconnection (and yet slow)
            composable --chain=composable --listen-addr=/ip4/0.0.0.0/tcp/30334 --prometheus-port 9615 --base-path ../data/composable-follow --execution=wasm --ws-external --state-pruning=archive --blocks-pruning=archive --rpc-external --rpc-cors=all --unsafe-rpc-external --rpc-methods=unsafe --ws-port 9988 --unsafe-ws-external --rpc-port 39988 --in-peers 1000 --out-peers 1000 --ws-max-connections 10000  --sync=full -- --execution=wasm --listen-addr=/ip4/0.0.0.0/tcp/30333 --sync=full  --state-pruning=archive --blocks-pruning=archive
          '';
        };
      };
    };
  };
}
