{ ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, ... }: {
    # 1. run hyperspae in process compose
    # 2. run centaurid in process compose
    # 3. run zombienet in process compose
    # 4. run run hyperspace with some configuraiton
    # 5. build hyperspace wasm
    # 6. upload hyperspace wasm
    # 7. run hyperspace with connected configuraiton
    process-compose.devnet-cosmos = {
      settings = {
        processes = {
          centauri = {
            command =self'.packages.banksyd-gen;
          };
          picasso = {
            command =self'.packages.banksyd-gen;
          };
        };
      };
    };
  };
}
