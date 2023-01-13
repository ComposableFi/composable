{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
   {
      packages = {
        help = pkgs.writeShellApplication {
          name = "help";
          runtimeInputs = with pkgs; [ glow ];
          text = ''
              glow ${./help.md}
          '';
        };
      };
    };
}