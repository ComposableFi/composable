{ self, withSystem, inputs, lib, options, flake-parts-lib, specialArgs, config
}: {
  flake = {
    nixopsConfigurations = withSystem "x86_64-linux"
      ({ config, self', inputs', pkgs, devnetTools, this, ... }:
        let
          getStringEnvOrDefault = name: default:
            if (builtins.getEnv name) != "" then
              (builtins.getEnv name)
            else
              default;
          service-account-credential-key-file-input = builtins.fromJSON
            (builtins.readFile
              (builtins.getEnv "GOOGLE_APPLICATION_CREDENTIALS"));
          domainSuffix = getStringEnvOrDefault "NIXOPS_DEVNETS_DOMAIN_SUFFIX"
            "devnets.composablefinance.ninja";
          certificateEmail =
            getStringEnvOrDefault "NIXOPS_DEVNETS_CERTIFICATE_EMAIL"
            "hussein@composable.finance";
          gce-to-nix = { project_id, client_email, private_key, ... }: {
            project = project_id;
            serviceAccount = client_email;
            accessKey = private_key;
          };
          gce-input = gce-to-nix service-account-credential-key-file-input;
        in {
          default = let nixpkgs = self.inputs.nixpkgs;
          in import ../devnets/devnet.nix {
            inherit nixpkgs gce-input domainSuffix certificateEmail pkgs;
            devnet-dali = this.dali-dev-ops;
            devnet-picasso = this.picasso-dev-ops;
            docs = self'.packages.docs-static;
            rev = builtins.getEnv "DEPLOY_REVISION";
          };
        });
  };
}
