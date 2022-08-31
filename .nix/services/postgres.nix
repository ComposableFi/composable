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
    command = [ "-p" "${toString database.port}" ];
    ports = [ "${toString database.port}:${toString database.port}" ];
  };
}
