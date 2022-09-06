{ database, pkgs, ...}:
{
  service = {
    build.context = ../../../subsquid;
    build.dockerfile = ../../../subsquid/graphql.Dockerfile;
  };
}
