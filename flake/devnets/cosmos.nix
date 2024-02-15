{ pkgs, devnet-root-directory, self', chain-restart, parachain-startup, relay
, devnetTools, networks, }:
let
  depends-on-cvm-init = {
    depends_on."centauri-cvm-init".condition = "process_completed_successfully";
    depends_on."osmosis-cvm-init".condition = "process_completed_successfully";
    depends_on."neutron-init".condition = "process_completed_successfully";
  };
in {
  settings = {
    log_level = "trace";
    log_location = "/tmp/composable-devnet/pc.log";
    processes = {
      centauri = {
        command = self'.packages.centaurid-gen;
        readiness_probe.http_get = {
          host = "127.0.0.1";
          port = networks.pica.devnet.CONSENSUS_RPC_PORT;
        };
        log_location = "${devnet-root-directory}/centauri.log";
        availability = { restart = chain-restart; };
      };

      neutron-init = {
        command = self'.packages.neutron-gen;
        log_location = "${devnet-root-directory}/neutron-init.log";
        availability = { restart = chain-restart; };
      };

      neutron = {
        command = self'.packages.neutron-start;
        readiness_probe.http_get = {
          host = "127.0.0.1";
          port = networks.neutron.devnet.CONSENSUS_RPC_PORT;
        };
        log_location = "${devnet-root-directory}/neutron-start.log";
        availability = { restart = chain-restart; };
        depends_on."neutron-init".condition = "process_completed_successfully";
      };

      cosmos-hub-init = {
        command = self'.packages.cosmos-hub-gen;
        log_location = "${devnet-root-directory}/cosmos-hub-init.log";
        availability = { restart = chain-restart; };
        namespace = "full-node";
      };

      cosmos-hub = {
        command = self'.packages.cosmos-hub-start;
        readiness_probe.http_get = {
          host = "127.0.0.1";
          port = networks.cosmos-hub.devnet.CONSENSUS_RPC_PORT;
        };
        log_location = "${devnet-root-directory}/cosmos-hub-start.log";
        availability = { restart = chain-restart; };
        depends_on."cosmos-hub-init".condition =
          "process_completed_successfully";
        namespace = "full-node";
      };

      neutron-cosmos-hub-init = {
        command = self'.packages.neutron-cosmos-hub-hermes-init;
        log_location = "${devnet-root-directory}/neutron-cosmos-hub-init.log";
        availability = { restart = relay; };
        depends_on."neutron".condition = "process_healthy";
        depends_on."cosmos-hub".condition = "process_healthy";
        namespace = "trustless-relay";
      };

      neutron-cosmos-hub-relay = {
        command = self'.packages.neutron-cosmos-hub-hermes-relay;
        log_location = "${devnet-root-directory}/neutron-cosmos-hub-relay.log";
        availability = { restart = relay; };
        depends_on."neutron".condition = "process_healthy";
        depends_on."cosmos-hub".condition = "process_healthy";
        depends_on."neutron-cosmos-hub-init".condition =
          "process_completed_successfully";
        namespace = "trustless-relay";
      };

      centauri-neutron-init = {
        command = self'.packages.neutron-centauri-hermes-init;
        log_location = "${devnet-root-directory}/centauri-neutron-init.log";
        availability = { restart = relay; };
        depends_on."neutron".condition = "process_healthy";
        depends_on."osmosis-centauri-init".condition =
          "process_completed_successfully";
        depends_on."neutron-cosmos-hub-init".condition =
          "process_completed_successfully";
        namespace = "trustless-relay";
      };

      centauri-cosmos-hub-init = {
        command = self'.packages.centauri-cosmos-hub-hermes-init;
        log_location = "${devnet-root-directory}/centauri-cosmos-hub-init.log";
        availability = { restart = relay; };
        depends_on."centauri".condition = "process_healthy";
        depends_on."cosmos-hub".condition = "process_healthy";
        namespace = "trustless-relay";
      };

      centauri-cosmos-hub-relay = {
        command = self'.packages.centauri-cosmos-hub-hermes-relay;
        log_location = "${devnet-root-directory}/cosmos-hub-centauri-relay.log";
        availability = { restart = relay; };
        depends_on."cosmos-hub".condition = "process_healthy";
        depends_on."centauri-cosmos-hub-init".condition =
          "process_completed_successfully";
        namespace = "trustless-relay";
      };

      osmosis-cosmos-hub-init = {
        command = self'.packages.osmosis-cosmos-hub-hermes-init;
        log_location = "${devnet-root-directory}/osmosis-cosmos-hub-init.log";
        availability = { restart = relay; };
        depends_on."osmosis".condition = "process_healthy";
        depends_on."cosmos-hub".condition = "process_healthy";
        namespace = "trustless-relay";
      };

      osmosis-cosmos-hub-relay = {
        command = self'.packages.osmosis-cosmos-hub-hermes-relay;
        log_location = "${devnet-root-directory}/cosmos-hub-osmosis-relay.log";
        availability = { restart = relay; };
        depends_on."cosmos-hub".condition = "process_healthy";
        depends_on."osmosis-cosmos-hub-init".condition =
          "process_completed_successfully";
        namespace = "trustless-relay";
      };

      centauri-neutron-relay = {
        command = self'.packages.centauri-neutron-hermes-relay;
        log_location = "${devnet-root-directory}/neutron-centauri-relay.log";
        availability = { restart = relay; };
        depends_on."neutron".condition = "process_healthy";
        depends_on."neutron-centauri-init".condition =
          "process_completed_successfully";
        namespace = "trustless-relay";
      };

      centauri-init = {
        command = self'.packages.centaurid-init;
        depends_on."centauri".condition = "process_healthy";
        log_location = "${devnet-root-directory}/centauri-init.log";
        availability = { restart = chain-restart; };
      };

      centauri-cvm-init = {
        command = self'.packages.centaurid-cvm-init;
        depends_on."centauri".condition = "process_healthy";
        log_location = "${devnet-root-directory}/centauri-cvm-init.log";
        availability = { restart = chain-restart; };
      };

      centauri-cvm-config = {
        command = self'.packages.centaurid-cvm-config;
        log_location = "${devnet-root-directory}/centauri-cvm-config.log";
        availability = { restart = chain-restart; };
      } // depends-on-cvm-init;

      osmosis-cvm-config = {
        command = self'.packages.osmosisd-cvm-config;
        log_location = "${devnet-root-directory}/osmosis-cvm-config.log";
        availability = { restart = chain-restart; };
      } // depends-on-cvm-init;

      neutron-cvm-config = {
        command = self'.packages.neutrond-cvm-config;
        log_location = "${devnet-root-directory}/neutron-cvm-config.log";
        availability = { restart = chain-restart; };
      } // depends-on-cvm-init;

      osmosis = {
        command = self'.packages.osmosisd-gen;
        readiness_probe.http_get = {
          host = "127.0.0.1";
          port = pkgs.networksLib.osmosis.devnet.CONSENSUS_RPC_PORT;
        };
        log_location = "${devnet-root-directory}/osmosis.log";
      };
      osmosis-pools-init = {
        command = self'.packages.osmosisd-pools-init;
        depends_on."osmosis".condition = "process_healthy";
        log_location = "${devnet-root-directory}/osmosisd-pools-init.log";
        availability = { restart = chain-restart; };
      };
      osmosis-cvm-init = {
        command = self'.packages.osmosis-cvm-init;
        depends_on."osmosis".condition = "process_healthy";
        log_location = "${devnet-root-directory}/osmosis-cvm-init.log";
        availability = { restart = chain-restart; };
        namespace = "osmosis";
      };

      osmosis-centauri-init = {
        command = self'.packages.osmosis-centauri-hermes-init;
        depends_on = {
          "centauri-init".condition = "process_completed_successfully";
          "osmosis".condition = "process_healthy";
        };
        namespace = "trustless-relay";
        log_location = "${devnet-root-directory}/osmosis-centauri-init.log";
        availability = { restart = relay; };
      };

      osmosis-centauri-relay = {
        command = self'.packages.osmosis-centauri-hermes-relay;
        depends_on = {
          "osmosis-centauri-init".condition = "process_completed_successfully";
        };
        log_location = "${devnet-root-directory}/osmosis-centauri-relay.log";
        availability = { restart = relay; };
        namespace = "trustless-relay";
      };

      mantis-simulate-solve = {
        command = self'.packages.mantis-simulate-solve;
        depends_on = {
          "centauri-cvm-config".condition = "process_completed_successfully";
          "osmosis-centauri-init".condition = "process_completed_successfully";
        };
        log_location = "${devnet-root-directory}/mantis-simulate-solve.log";
        availability = { restart = "on_failure"; };
        namespace = "xapp";
      };
    };
  };
}
