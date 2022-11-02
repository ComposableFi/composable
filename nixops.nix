{ self, withSystem, ... }: {
  flake = {
    nixopsConfigurations = withSystem "x86_64-linux" ({ config, self', inputs', pkgs, devnetTools, ... }: let 

      service-account-credential-key-file-input = builtins.fromJSON
        (builtins.readFile (builtins.getEnv "GOOGLE_APPLICATION_CREDENTIALS"));

      gce-to-nix = { project_id, client_email, private_key, ... }: {
        project = project_id;
        serviceAccount = client_email;
        accessKey = private_key;
      };

      gce-input = gce-to-nix service-account-credential-key-file-input;
    in {
      default =
        let 
          nixpkgs = self.inputs.nixpkgs;
          inherit pkgs;
        in
        import ./.nix/devnet.nix {
          inherit nixpkgs;
          inherit gce-input;
          devnet-dali = pkgs.callPackage devnetTools.mk-devnet {
            inherit (self'.packages)
              polkadot-launch composable-node polkadot-node;
            chain-spec = "dali-dev";
          };
          devnet-picasso = pkgs.callPackage devnetTools.mk-devnet {
            inherit (self'.packages)
              polkadot-launch composable-node polkadot-node;
            chain-spec = "picasso-dev";
          };
          docs = self.packages'.docs-static;
          rev = builtins.getEnv "GITHUB_SHA";
        };
    });
  };
}
