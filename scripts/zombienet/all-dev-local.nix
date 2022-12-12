{
  hrmp_channels = [
    {
      max_capacity = 8;
      max_message_size = 512;
      recipient = 1000;
      sender = 2087;
    }
    {
      max_capacity = 8;
      max_message_size = 512;
      recipient = 2087;
      sender = 1000;
    }
  ];
  parachains = [
    {
      add_to_genesis = true;
      chain = "statemine-local";
      collators = [
        {
          command =
            "../../../../paritytech/cumulus/target/release/polkadot-parachain";
          env = [{
            name = "RUST_LOG";
            value =
              "runtime=debug,parachain=trace,cumulus-collator=trace,aura=trace,xcm=trace";
          }];
          name = "statemine-local-alice";
          rpc_port = 32220;
          validator = true;
          ws_port = 10008;
        }
        {
          command =
            "../../../../paritytech/cumulus/target/release/polkadot-parachain";
          env = [{
            name = "RUST_LOG";
            value =
              "runtime=debug,parachain=trace,cumulus-collator=trace,aura=trace,xcm=trace";
          }];
          name = "statemine-local-bob";
          validator = true;
        }
      ];
      cumulus_based = true;
      genesis = {
        runtime = {
          balances = {
            balances = {
              "0" = {
                "0" = "5GF8HUxZ1tCHWVZMoGVqDWe8EtdnKznHEbaguRQNqvMkw34H";
                "1" = 1123000000000000000;
              };
            };
          };
        };
      };
      id = 1000;
    }
    {
      add_to_genesis = true;
      chain = "dali-dev";
      collators = [
        {
          command = "../../target/release/composable";
          env = [{
            name = "RUST_LOG";
            value =
              "runtime=debug,parachain=trace,cumulus-collator=trace,aura=trace,xcm=trace";
          }];
          name = "alice";
          rpc_port = 32200;
          validator = true;
          ws_port = 9988;
        }
        {
          command = "../../target/release/composable";
          env = [{
            name = "RUST_LOG";
            value =
              "runtime=debug,parachain=trace,cumulus-collator=trace,aura=trace,xcm=trace";
          }];
          name = "bob";
          validator = true;
        }
        {
          command = "../../target/release/composable";
          env = [{
            name = "RUST_LOG";
            value =
              "runtime=debug,parachain=trace,cumulus-collator=trace,aura=trace,xcm=trace";
          }];
          name = "charlie";
          validator = true;
        }
      ];
      cumulus_based = true;
      id = 2087;
    }
  ];
  relaychain = {
    chain = "rococo-local";
    default_args = [ "-lparachain=debug" ];
    default_command = "../../../../paritytech/polkadot/target/release/polkadot";
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
    nodes = [
      {
        name = "rococo-local-alice";
        rpc_port = 30444;
        validator = true;
        ws_port = 9944;
      }
      {
        name = "rococo-local-bob";
        validator = true;
      }
      {
        name = "rococo-local-charlie";
        validator = true;
      }
      {
        name = "rococo-local-dave";
        validator = true;
      }
    ];
  };
  settings = {
    node_spawn_timeout = 120;
    provider = "native";
    timeout = 600;
  };
  types = {
    Header = {
      number = "u64";
      parent_hash = "Hash";
      post_state = "Hash";
    };
  };
}
