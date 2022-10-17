{ pkgs, juno, database, junoPort }: {
  service = let
    src = pkgs.fetchFromGitHub {
      owner = "hussein-aitlahcen";
      repo = "cosmos-subql-starter";
      rev = "33fe16f70ddcf6b418a06d3194a581cd67ae7d0d";
      hash = "sha256-sQXVJZ6OSKPa11n601QFMchLSzlKepMCyn082VYZyzg=";
    };
    subql = pkgs.mkYarnPackage {
      inherit src;
      yarnLock = "${src}/yarn.lock";
      buildPhase = ''
        runHook preBuild
        yarn codegen
        yarn build
        runHook postBuild
      '';
      installPhase = ''
        runHook preInstall
        mkdir $out
        cp -r $src/* $out/
        cp -r deps/*/dist $out/
        runHook postInstall
      '';
      distPhase = ":";
    };
    context = let
      files = pkgs.linkFarm "context" [
        {
          name = "subql";
          path = subql;
        }
        {
          name = "Dockerfile";
          path = pkgs.writeText "Dockerfile" ''
            FROM onfinality/subql-node-cosmos:v0.2.0
            COPY subql /app
          '';
        }
      ];
    in pkgs.stdenv.mkDerivation {
      name = "context";
      phases = [ "installPhase" ];
      installPhase = ''
        mkdir $out
        cp -rL ${files}/* $out
      '';
    };
  in {
    build = { context = "${context}"; };
    restart = "always";
    environment = {
      DB_USER = database.user;
      DB_PASS = database.password;
      DB_DATABASE = database.name;
      DB_HOST = database.host;
      DB_PORT = database.port;
    };
    # NOTE: not found yet cors or how to set cors on express/nextjs
    command = [
      "-f=/app"
      "--db-schema=cosmos"
      "--network-endpoint=http://${juno}:${toString junoPort}"
    ];
  };
}
