{ pkgs, version, database, init-scripts }:
let
  context = let
    files = pkgs.linkFarm "context" [
      {
        name = "init-scripts";
        path = init-scripts;
      }
      {
        name = "Dockerfile";
        path = pkgs.writeText "Dockerfile" ''
          FROM postgres:${version}
          COPY init-scripts /docker-entrypoint-initdb.d
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
  service = {
    build = { context = "${context}"; };
    environment = {
      POSTGRES_USER = database.user;
      POSTGRES_DB = database.name;
      POSTGRES_PASSWORD = database.password;
    };
    restart = "always";
    healthcheck = {
      test =
        [ "CMD-SHELL" "pg_isready -d ${import ../util/db-url.nix database}" ];
      interval = "5s";
      timeout = "5s";
      retries = 5;
    };

    command = [ "-p" "${toString database.port}" ];
    ports = [ "${toString database.port}:${toString database.port}" ];
  };
}
