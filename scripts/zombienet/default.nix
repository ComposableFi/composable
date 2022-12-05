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
in with prelude; {
  mkChannel = sender: recipient: [{
    max_capacity = 8;
    max_message_size = 4096;
    recipient = recipient;
    sender = sender;
  }];

  mkBidirectionalChannel = a: b: (mkChannel a b) ++ (mkChannel b a);

  mkCollator = { command, name ? "alice", rpc_port ? null, ws_port ? null }:
    {
      command = command;
      env = [{
        name = "RUST_LOG";
        value =
          "runtime=debug,parachain=trace,cumulus-collator=trace,aura=trace,xcm=trace";
      }];
      name = name;
      validator = true;
    } // optionalAttrs (rpc_port != null) { inherit rpc_port; }
    // optionalAttrs (ws_port != null) { inherit ws_port; };

  mkParachain = { command, rpc_port ? 32200, ws_port ? 9988, chain ? "dali-dev"
    , names ? default-node-names, collators ? 2, id ? 2087 }:
    let
      generated = lib.lists.zipListsWith (_increment: name:
        mkCollator {
          inherit command;
          inherit name;
        })

        (lib.lists.range 0 (count - 1)) (builtins.tail names);

    in {
      add_to_genesis = true;
      chain = chain;
      cumulus_based = true;
      id = id;
      collators = [
        mkCollator
        {
          inherit command;
          inherit rpc_port;
          inherit ws_port;
          inherit chain;
          name = builtins.head names;
        }
      ];
    };

  mkParachains = parachains: builtins.map mkParachain parachains;

  mkHrmpChannels = { parachains }:
    let
      ids = map (x: x.id) parachains;
      cross = pkgs.lib.cartesianProductOfSets {
        sender = ids;
        recipient = ids;
      };
      unique = filter (x: x.sender != x.recipient) cross;
    in map mkChannel unique;

  mkRelaychainNode = { rpc_port, ws_port, name }:
    {
      name = name;
      validator = true;
    } // optionalAttrs (rpc_port != null) { inherit rpc_port; }
    // optionalAttrs (ws_port != null) { inherit ws_port; };

  mkRelaychainNodes = { chain, rpc_port ? 30444, ws_port ? 9944, count ? 2 }:
    let
      prefixName = name: chain ++ "-" ++ name;
      generated = lib.lists.zipListsWith
        (_increment: name: mkRelaychainNode { name = prefixName name; })
        (lib.lists.range 0 (count - 1)) (builtins.tail names);
      bootstrap = mkRelaychainNode {
        ws_port = 9944;
        rpc_port = 30444;
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
      nodes = mkRelaychainNodes {
        inherit rpc_port;
        inherit ws_port;
        inherit count;
      };
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
  mkZombienet = { relay, parachains }:
    let
      # collators
      # relaychain
      # settings 
      # types
    in {

    };
}
