# allows to compose DSL to instantiate relay and parachains
{ pkgs }:
let
  default-node-names = [ "alice" "bob" "charlie" "dave" "eve" "ferdie" ];
  prelude = rec {
    lib = pkgs.lib;
    optionalAttrs = lib.optionalAttrs;
    map = builtins.map;
    filter = builtins.filter;
  };
in with prelude; rec {
  mkChannel = sender: recipient: {
    max_capacity = 8;
    max_message_size = 4096;
    recipient = recipient;
    sender = sender;
  };

  mkBidirectionalChannel = a: b: (mkChannel a b) ++ (mkChannel b a);

  mkCollator = { name ? "alice", command, rpc_port ? null, ws_port ? null
    , rust_log_add ? "" }:
    {
      command = command;
      args = [
        "--wasmtime-instantiation-strategy=recreate-instance-copy-on-write"
        "--enable-offchain-indexing=true"
      ];
      env = [{
        name = "RUST_LOG";
        value =
          "info,runtime=debug,parachain=trace,cumulus-collator=trace,aura=trace,xcm=trace,pallet_ibc=trace"
          + (if rust_log_add != null then rust_log_add else "");
      }];
      name = name;
      validator = true;
    } // optionalAttrs (rpc_port != null) { inherit rpc_port; }
    // optionalAttrs (ws_port != null) { inherit ws_port; };

  mkParachain = { command, rpc_port ? 32200, ws_port ? 9988, chain ? "dali-dev"
    , names ? default-node-names, collators ? 2, id ? 2087, rust_log_add ? null
    }:
    let
      generated = lib.lists.zipListsWith
        (_increment: name: mkCollator { inherit command name rust_log_add; })
        (lib.lists.range 0 (collators - 2)) (builtins.tail names);

    in {
      add_to_genesis = true;
      chain = chain;
      cumulus_based = true;
      id = id;
      collators = [
        (mkCollator {
          inherit command rust_log_add rpc_port ws_port;
          name = builtins.head names;
        })
      ] ++ generated;
    };

  mkParachains = parachains: builtins.map mkParachain parachains;

  mkHrmpChannels = parachains:
    let
      ids = map (x: x.id) parachains;
      cross = pkgs.lib.cartesianProductOfSets {
        sender = ids;
        recipient = ids;
      };
      unique = filter (x: x.sender != x.recipient) cross;
    in map (pair: mkChannel pair.sender pair.recipient) unique;

  mkRelaychainNode = { rpc_port ? null, ws_port ? null, name }:
    {
      name = name;
      validator = true;
    } // optionalAttrs (rpc_port != null) { inherit rpc_port; }
    // optionalAttrs (ws_port != null) { inherit ws_port; };

  mkRelaychainNodes = { chain, rpc_port ? 30444, ws_port ? 9944, count ? 2
    , names ? default-node-names }:
    let
      prefixName = name: "${chain}-${name}";
      generated = lib.lists.zipListsWith
        (_increment: name: mkRelaychainNode { name = prefixName name; })
        (lib.lists.range 0 (count - 1)) (builtins.tail names);
      bootstrap = mkRelaychainNode {
        inherit ws_port rpc_port;
        name = prefixName "alice";
      };
    in [ bootstrap ] ++ generated;

  mkRelaychain =
    { chain, default_command, rpc_port ? 30444, ws_port ? 9944, count ? 2 }: {
      inherit default_command;
      inherit chain;
      default_args = [ "-lparachain=debug" ];
      genesis = {
        runtime = {
          runtime_genesis_config = {
            configuration = {
              config = {
                max_validators_per_core = 2;
                needed_approvals = 1;
                validation_upgrade_cooldown = 2;
                validation_upgrade_delay = 2;
              };
            };
          };
        };
      };
      nodes = mkRelaychainNodes { inherit rpc_port ws_port count chain; };
    };
  mkSettings = {
    node_spawn_timeout = 120;
    provider = "native";
    timeout = 600;
  };
  mkTypes = {
    Header = {
      number = "u64";
      parent_hash = "Hash";
      post_state = "Hash";
    };
  };
  mkZombienet = { relaychain, parachains }: {
    hrmp_channels = mkHrmpChannels parachains;
    relaychain = mkRelaychain relaychain;
    parachains = mkParachains parachains;
    settings = mkSettings;
    types = mkTypes;
  };

  zombienet-to-ops = zombienet:
    # output network information in a format that ops(compose, deploy, tests) can consume
    let
      ops-node = { name ? null, ws_port ? null, ... }:
        if name != null && ws_port != null then {
          ws_port = ws_port;
          name = name;
        } else
          null;

      driedCollators = collators:
        builtins.filter (e: e != null) (builtins.map ops-node collators);
      driedParachains = parachains:
        builtins.map (e: driedCollators e.collators) parachains;
    in {
      parachain-nodes = driedParachains zombienet.parachains;
      relaychain-nodes = builtins.filter (e: e != null)
        (builtins.map ops-node zombienet.relaychain.nodes);
    };
}
