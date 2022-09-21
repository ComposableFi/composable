# allows to compose DSL to instantiate relay and parachains
# 
# package operates in next assumptions:
# 1. we good to have all nodes to be equal regarding logging/storage/networking per chain
# 2. allocate node names equal to owners sudo keys according well know keyring
# 3. it is possible to allocate range of ports to each node starting from some base
{ pkgs }:
with pkgs;
let
  default-flags = [
    "--unsafe-ws-external"
    "--unsafe-rpc-external"
    "--rpc-external"
    "--ws-external"
    "--rpc-cors=all"
    "--rpc-methods=Unsafe"
    "--execution=wasm"
    "--wasmtime-instantiation-strategy=recreate-instance-copy-on-write"
  ];

  default-node-names = [ "alice" "bob" "charlie" "dave" "eve" "ferdie" ];
in rec {
  mk-node = { port, wsPort, nodeKey, flags, basePath }: {
    name = nodeKey;
    rpcPort = port + 1000;
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

  mk-relaychain = { bin, chain, port, wsPort, count
    , nodeNames ? default-node-names, flags ? [ ] }:
    mk-chain {
      inherit bin;
      inherit chain;
      inherit port;
      inherit wsPort;
      inherit count;
      inherit nodeNames;
      flags = let
        mandatory-flags =
          [ "--rpc-cors=all" "--beefy" "--enable-offchain-indexing=true" ];
      in mandatory-flags
      ++ builtins.filter (flag: !(builtins.elem flag mandatory-flags)) flags;
    };

  mk-shared-security-network = { parachains, relaychain }: {
    parachains = mk-parachains parachains;
    relaychain = mk-relaychain relaychain;
    hrmpChannels = let
      map = builtins.map;
      filter = builtins.filter;
      ids = map (x: x.id) parachains;
      cross = pkgs.lib.cartesianProductOfSets {
        sender = ids;
        recipient = ids;
      };
      unique = filter (x: x.sender != x.recipient) cross;
    in map (connection: {
      sender = connection.sender;
      recipient = connection.recipient;
      maxCapacity = 8;
      maxMessageSize = 16384;
    }) unique;
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
