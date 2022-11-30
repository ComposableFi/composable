# allows to compose DSL to instantiate relay and parachains
# 
# package operates in next assumptions:
# 1. we good to have all nodes to be equal regarding logging/storage/networking per chain
# 2. allocate node names equal to owners sudo keys according well know keyring
# 3. it is possible to allocate range of ports to each node starting from some base
{ pkgs }:
let
  default-node-names = [ "alice" "bob" "charlie" "dave" "eve" "ferdie" ];
  prelude = rec {
    lib = pkgs.lib;
    optionalAttrs = lib.optionalAttrs;
  };
in with prelude; {
  mkChannel = sender: recipient: [{
    max_capacity = 8;
    max_message_size = 512;
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
  # mkRelaychainNode =
  # mkRelaychain =
  # mkZombienet =  
}
