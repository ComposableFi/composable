{ self, ... }: {
  perSystem = { pkgs, ... }:
   {
      packages = {
        help = pkgs.writeShellApplication {
          name = "help";
          runtimeInputs = [ pkgs.glow ];
          text = ''
              glow ${./help.md}
          '';
        };
      };
    };
}