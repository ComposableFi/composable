{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      mkFrontendStatic =
        { kusamaEndpoint, picassoEndpoint, karuraEndpoint, subsquidEndpoint }:
        pkgs.buildYarnPackage {
          nativeBuildInputs = [ pkgs.pkg-config pkgs.vips pkgs.python3 ];
          src = ./.;

          # The filters exclude the storybooks for faster builds
          yarnBuildMore =
            "yarn export --filter=pablo --filter=picasso --filter=!picasso-storybook --filter=!pablo-storybook";

          # TODO: make these configurable
          preBuild = ''
            export SUBSQUID_URL="${subsquidEndpoint}";

            # Polkadot
            export SUBSTRATE_PROVIDER_URL_KUSAMA_2019="${picassoEndpoint}";
            export SUBSTRATE_PROVIDER_URL_KUSAMA="${kusamaEndpoint}";
            export SUBSTRATE_PROVIDER_URL_KARURA="${karuraEndpoint}";
          '';
          installPhase = ''
            mkdir -p $out
            mkdir $out/pablo
            mkdir $out/picasso
            cp -R ./apps/pablo/out/* $out/pablo
            cp -R ./apps/picasso/out/* $out/picasso
          '';
        };
    in {
      packages = rec {
        frontend-static = mkFrontendStatic {
          subsquidEndpoint = "http://localhost:4350/graphql";
          picassoEndpoint = "ws://localhost:9988";
          kusamaEndpoint = "ws://localhost:9944";
          karuraEndpoint = "ws://localhost:9999";
        };

        frontend-static-persistent = mkFrontendStatic {
          subsquidEndpoint =
            "https://persistent.devnets.composablefinance.ninja/subsquid/graphql";
          picassoEndpoint =
            "wss://persistent.devnets.composablefinance.ninja/chain/dali";
          kusamaEndpoint =
            "wss://persistent.devnets.composablefinance.ninja/chain/rococo";
          karuraEndpoint =
            "wss://persistent.devnets.composablefinance.ninja/chain/karura";
        };

        frontend-static-picasso-persistent = mkFrontendStatic {
          subsquidEndpoint =
            "https://persistent.picasso.devnets.composablefinance.ninja/subsquid/graphql";
          picassoEndpoint =
            "wss://persistent.picasso.devnets.composablefinance.ninja/chain/picasso";
          kusamaEndpoint =
            "wss://persistent.picasso.devnets.composablefinance.ninja/chain/rococo";
          karuraEndpoint =
            "wss://persistent.picasso.devnets.composablefinance.ninja/chain/karura";
        };

        frontend-static-firebase = mkFrontendStatic {
          subsquidEndpoint =
            "https://persistent.devnets.composablefinance.ninja/subsquid/graphql";
          picassoEndpoint =
            "wss://persistent.devnets.composablefinance.ninja/chain/dali";
          kusamaEndpoint =
            "wss://persistent.devnets.composablefinance.ninja/chain/rococo";
          karuraEndpoint =
            "wss://persistent.devnets.composablefinance.ninja/chain/karura";
        };

        frontend-pablo-server = let PORT = 8002;
        in pkgs.writeShellApplication {
          name = "frontend-pablo-server";
          runtimeInputs = [ pkgs.miniserve ];
          text = ''
            miniserve -p ${
              builtins.toString PORT
            } --spa --index index.html ${frontend-static}/pablo
          '';
        };

        frontend-picasso-server = let PORT = 8003;
        in pkgs.writeShellApplication {
          name = "frontend-picasso-server";
          runtimeInputs = [ pkgs.miniserve ];
          text = ''
            miniserve -p ${
              builtins.toString PORT
            } --spa --index index.html ${frontend-static}/picasso
          '';
        };
      };
    };
}
