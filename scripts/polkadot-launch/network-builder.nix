# allows to compose DSL to instantiate relay and parachains
# 
# package operates in next assumptions:
# 1. we good to have all nodes to be equal regarding logging/storage/networking per chain
# 2. alloacte node names equal to owners sudo keys according well know keyring
# 3. it is possible to allocate range of prots to each node starting from some base
{ pkgs }:
with pkgs;
let
  default-flags = [
    "--rpc-cors=all"
    "--wasmtime-instantiation-strategy=recreate-instance-copy-on-write"
    "--"
    "--execution=wasm"
  ];
  default-node-names = [ "alice" "bob" "charlie" "dave" "eve" "ferdie" ];
in rec {
  mk-node = { port, wsPort, nodeKey, flags, basePath }: {
    name = nodeKey;
    inherit flags;
    inherit port;
    inherit wsPort;
    inherit basePath;
  };

  mk-nodes = { count, port, wsPort, nodeNames, flags, basePath }:
    let portsIncrements = lib.lists.range 0 (count - 1);
    in lib.lists.zipListsWith (portIncrement: nodeKey:
      mk-node {
        port = port + portIncrement;
        wsPort = wsPort + portIncrement;
        inherit nodeKey;
        inherit flags;
        basePath = "${basePath}/${nodeKey}";
      }) portsIncrements nodeNames;
  mk-chain = { bin, chain, port, wsPort, count, nodeNames, flags }: {
    inherit chain;
    inherit bin;
    nodes = mk-nodes {
      inherit count;
      inherit port;
      inherit wsPort;
      inherit nodeNames;
      inherit flags;
      basePath = "/tmp/polkadot-launch/${chain}/";
    };
  };

  mk-parachain = { balance ? "1000000000000000000000", bin, chain, id, port
    , wsPort, count, nodeNames ? default-node-names, flags ? default-flags }:
    {
      inherit balance;
      inherit id;
    } // mk-chain {
      inherit bin;
      inherit chain;
      inherit port;
      inherit wsPort;
      inherit count;
      inherit nodeNames;
      inherit flags;
    };

  # here we can add overrides per spec, example for flags
  mk-parachains = specs: builtins.map mk-parachain specs;

  mk-relaychain =
    { bin, chain, port, wsPort, count, nodeNames ? default-node-names }:
    mk-chain {
      inherit bin;
      inherit chain;
      inherit port;
      inherit wsPort;
      inherit count;
      inherit nodeNames;
      flags = [ "--rpc-cors=all" "--beefy" "--enable-offchain-indexing=true" ];
    };

  mk-shared-security-network = { parachains, relaychain }: {
    parachains = mk-parachains parachains;
    relaychain = mk-relaychain relaychain;
    genesis = {
      runtime = {
        runtime_genesis_config = {
          configuration = {
            config = {
              validation_upgrade_frequency = 2;
              validation_upgrade_delay = 2;
            };
          };
        };
      };
    };
  };
}
