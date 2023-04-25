{ self, ... }: {
  perSystem =
    { config
    , self'
    , inputs'
    , pkgs
    , system
    , crane
    , systemCommonRust
    , cargoTools
    , ...
    }: {
      packages =
        let
          pkgs-latest = self.inputs.nixpkgs-latest.legacyPackages.${system};
          config = ./agent-config.yaml;
          grafana-observe =
                pkgs.writeShellApplication {
                name = "grafana-observe";
                runtimeInputs = [ pkgs-latest.grafana-agent ];
                text =
                  "grafana-agent --config.file=${config}";
              };
        in
        {
          grafana-agent = pkgs-latest.grafana-agent;
          inherit grafana-observe;
        };


      apps = { };

    };
}
