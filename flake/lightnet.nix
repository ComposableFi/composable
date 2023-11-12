{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, ... }:

    {
      packages = {
        lightnet-picasso = pkgs.writeShellApplication {
          name = "lightnet-picasso";
          runtimeInputs = [ pkgs.nodejs ];
          text = ''
            npx @acala-network/chopsticks@latest xcm -r kusama -p picasso-kusama -p statemine
          '';
        };
      };
    };
}
