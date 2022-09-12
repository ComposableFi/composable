{ database, polkadotEndpoint, prometheusPort }: {
  service = {
    # dependsOn = [ db-container-name ];
    restart = "on-failure";
    image = "subsquid/substrate-ingest:firesquid";
    command = [

      # polkadot endpoints -- replace with your wss
      "-e"
      polkadotEndpoint
      "-c"
      "10" # allow up to 20 pending requests for the above endpoint (defa>
      #  "--start-block", "1000000", # uncomment to specify a non-zero start blo>
      "--prom-port"
      "9090"
      "--out"
      (import ../util/db-url.nix database)
    ];
    ports = [ "${toString prometheusPort}:9090" ];
  };
}
