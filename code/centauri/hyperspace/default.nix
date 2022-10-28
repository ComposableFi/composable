{ pkgs, crane, common-attrs, common-deps }: {
  template = { relaychainHostA ? "127.0.0.1", relaychainPortA ? 9944
    , parachainHostA ? "127.0.0.1", parachainPortA ? 9988, paraIdA ? 2001
    , commitmentPrefixesA ? "0x6962632f", clientIdA ? "11-beefy-0"

    , relaychainHostB ? "127.0.0.1", relaychainPortB ? 9944
    , parachainHostB ? "127.0.0.1", parachainPortB ? 9188, paraIdB ? 2000
    , commitmentPrefixesB ? "0x6962632f", clientIdB ? "11-beefy-0", }: rec {
      bin = crane.buildPackage (common-attrs // {
        name = "hyperspace";
        pname = "hyperspace";
        cargoArtifacts = common-deps;
        cargoExtraArgs = "--package hyperspace";
        installPhase = ''
          mkdir --parents $out/bin
          cp target/release/hyperspace $out/bin/hyperspace
        '';
        meta = { mainProgram = "hyperspace"; };
      });
      rawConfig = (builtins.fromTOML (builtins.readFile ./config.toml));

      config =
        pkgs.lib.makeOverridable (result: { result = result; }) rawConfig;

      toStr = builtins.toString;

      configureChain = { self, relaychainHost, relaychainPort, parachainHost
        , parachainPort, paraId, commitmentPrefixes, clientId }:
        (self // {
          "parachain_rpc_url" =
            "ws://${relaychainHost}:${toStr relaychainPort}";
          "relay_chain_rpc_url" =
            "ws://${parachainHost}:${toStr parachainPort}";
          "para_id" = paraId;
          "commitment_prefix" = commitmentPrefixes;
          "client_id" = clientId;
        });

      default-config = pkgs.writeTextFile {
        name = "hyperspace.local.config.json";
        text = "${builtins.toJSON (config.override (self:
          self // {
            chain_a = configureChain {
              self = self.chain_a;
              relaychainHost = relaychainHostA;
              relaychainPort = relaychainPortA;
              parachainHost = parachainHostA;
              parachainPort = parachainPortA;
              paraId = paraIdA;
              commitmentPrefixes = commitmentPrefixesA;
              clientId = clientIdA;
            };

            chain_b = configureChain {
              self = self.chain_b;
              relaychainHost = relaychainHostB;
              relaychainPort = relaychainPortB;
              parachainHost = parachainHostB;
              parachainPort = parachainPortB;
              paraId = paraIdB;
              commitmentPrefixes = commitmentPrefixesB;
              clientId = clientIdB;
            };
          })).result}";
      };

      default = pkgs.writeShellApplication {
        name = "default-hyperspace";
        runtimeInputs = [ pkgs.coreutils pkgs.bash ];
        text = ''
          ${pkgs.yq}/bin/yq  . ${default-config} --toml-output > hyperspace.local.config.toml
          cat hyperspace.local.config.toml
          ${
            pkgs.lib.meta.getExe bin
          } relay --config hyperspace.local.config.toml
        '';
      };
    };
}
