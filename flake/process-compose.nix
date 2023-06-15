{ ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, ... }: {
    # 1. run hyperspae in process compose
    # 2. run centaurid in process compose
    # 3. run zombienet in process compose
    # 4. run run hyperspace with some configuraiton
    # 5. build hyperspace wasm
    # 6. upload hyperspace wasm
    # 7. run hyperspace with connected configuraiton



    process-compose.devnet-cosmos =
      let
 a = 1;
      in

      {
        settings = {
          processes = {
            sleep = {
              command =''
              sleep 100
              '';
            };
            # centauri = {
            #   command =self'.packages.centaurid-gen;
            # };
            # picasso = {
            #   command =self'.packages.zombienet-rococo-local-picasso-dev;
            # };
            hyperspace = {
              command = ''
                COMPOSABLE_DATA=/tmp/composable-devnet/
                HYPERSPACE_DATA="$COMPOSABLE_DATA/hyperspace"
                mkdir --parents "$COMPOSABLE_DATA"
                mkdir --parents "$HYPERSPACE_DATA"
                
                cp -f ${self'.packages.hyperspace-config-chain-a} $HYPERSPACE_DATA/config-chain-a.toml  
                cp -f ${self'.packages.hyperspace-config-chain-b} $HYPERSPACE_DATA/config-chain-b.toml  
                cp -f ${self'.packages.hyperspace-config-chain-2} $HYPERSPACE_DATA/config-chain-c.toml  
                cp -f ${self'.packages.hyperspace-config-core} $HYPERSPACE_DATA/config-core.toml                
                ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-clients --
            '';
            };
          };
        };
      };
  };
}
