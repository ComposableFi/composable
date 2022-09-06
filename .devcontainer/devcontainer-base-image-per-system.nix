# https://github.com/microsoft/vscode-dev-containers/tree/main/containers/debian
# https://github.com/numtide/flake-utils/blob/master/default.nix
# nix-prefetch-docker --image-name mcr.microsoft.com/vscode/devcontainers/base  --image-tag 0.202.7-bullseye --os linux --arch arm64
system:
let
  base-images = {
    # NOTE: we peeky container version to make these work version compatible with nix and native packaging/scripts
    x86_64-linux = {
      arch = "amd64";
      sha256 = "0vraf6iwbddpcy4l9msks6lmi35k7wfgpafikb56k3qinvvcjm9b";
      imageDigest =
        "sha256:269cbbb2056243e2a88e21501d9a8166d1825d42abf6b67846b49b1856f4b133";
    };
    x86_64-darwin = {
      arch = "amd64";
      sha256 = "0vraf6iwbddpcy4l9msks6lmi35k7wfgpafikb56k3qinvvcjm9b";
      imageDigest =
        "sha256:269cbbb2056243e2a88e21501d9a8166d1825d42abf6b67846b49b1856f4b133";
    };

    aarch64-linux = {
      arch = "arm64";
      imageDigest =
        "sha256:269cbbb2056243e2a88e21501d9a8166d1825d42abf6b67846b49b1856f4b133";
      sha256 = "0ddf1qmbykh7vvpxwgpzkxszgrq5k7xmhapjazwyjvs6m44c65jc";
    };
    aarch64-darwin = {
      arch = "arm64";
      imageDigest =
        "sha256:269cbbb2056243e2a88e21501d9a8166d1825d42abf6b67846b49b1856f4b133";
      sha256 = "0ddf1qmbykh7vvpxwgpzkxszgrq5k7xmhapjazwyjvs6m44c65jc";
    };
  };
in if builtins.hasAttr system base-images then
  base-images.${system}
else
  base-images.x86_64-linux
