{ pkgs, database }: {
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
  in {
    name = "cosmos-subql";
    image = "onfinality/subql-node-cosmos:v0.2.0";
    restart = "always";
    network_mode = "host";
    environment = {
      DB_USER = database.user;
      DB_PASS = database.password;
      DB_DATABASE = database.name;
      DB_HOST = database.host;
      DB_PORT = database.port;
    };
    command = [
      "-f=/app"
      "--db-schema=cosmos"
      "--network-endpoint=http://127.0.0.1:26657"
    ];
    volumes = [ "${subql}/:/app" ];
  };
}
