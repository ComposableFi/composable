{ pkgs, devnet-root-directory, self', chain-restart, parachain-startup, relay
, devnetTools, }:

{
  settings = {
    log_level = "trace";
    log_location = "/tmp/composable-devnet/pc.log";
    processes = {
      centauri = {
        command = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools;
          name = "centauri";
          text = ''
            ${pkgs.lib.meta.getExe self'.packages.centaurid-gen} reuse 0
          '';
        };
        readiness_probe.http_get = {
          host = "127.0.0.1";
          port = 26657;
        };
        log_location = "${devnet-root-directory}/centauri.log";
        availability = { restart = chain-restart; };
        namespace = "cosmos";
      };
      centauri-init = {
        command = self'.packages.centaurid-init;
        depends_on."centauri".condition = "process_healthy";
        log_location = "${devnet-root-directory}/centauri-init.log";
        availability = { restart = chain-restart; };
        namespace = "cosmos";
      };

      picasso = {
        command = self'.packages.zombienet-rococo-local-picasso-dev;
        availability = { restart = chain-restart; };
        log_location = "${devnet-root-directory}/picasso.log";
        readiness_probe = {
          exec.command = ''
            curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "assets_listAssets"}' http://localhost:9988
          '';
        } // parachain-startup;
        namespace = "polkadot";
      };

      picasso-centauri-ibc-init = {
        command = self'.packages.picasso-centauri-ibc-init;
        log_location = "${devnet-root-directory}/picasso-centauri-ibc-init.log";
        depends_on = {
          "centauri-init".condition = "process_completed_successfully";
          "centauri".condition = "process_healthy";
          "picasso".condition = "process_healthy";
        };
        availability = { restart = relay; };
      };

      picasso-centauri-ibc-connection-init = {
        command = self'.packages.picasso-centauri-ibc-connection-init;
        log_location =
          "${devnet-root-directory}/picasso-centauri-ibc-connection-init.log";
        depends_on = {
          "picasso-centauri-ibc-init".condition =
            "process_completed_successfully";
        };
        availability = { restart = relay; };
      };

      picasso-centauri-ibc-channels-init = {
        command = self'.packages.picasso-centauri-ibc-channels-init;
        log_location =
          "${devnet-root-directory}/picasso-centauri-ibc-channels-init.log";
        depends_on = {
          "picasso-centauri-ibc-connection-init".condition =
            "process_completed_successfully";
          "picasso-centauri-ibc-init".condition =
            "process_completed_successfully";
        };
        availability = { restart = relay; };
      };

      picasso-centauri-ibc-relay = {
        command = self'.packages.picasso-centauri-ibc-relay;
        log_location =
          "${devnet-root-directory}/picasso-centauri-ibc-relay.log";
        depends_on = {
          "picasso-centauri-ibc-channels-init".condition =
            "process_completed_successfully";
          "picasso-centauri-ibc-connection-init".condition =
            "process_completed_successfully";
          "picasso-centauri-ibc-init".condition =
            "process_completed_successfully";
        };
        availability = { restart = relay; };
      };

    };
  };
}
