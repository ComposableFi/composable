# https://github.com/microsoft/vscode-dev-containers/tree/main/containers/debian
# https://github.com/numtide/flake-utils/blob/master/default.nix
system:
let
  base-images = {
    # NOTE: we peeky container version to make these work version compatible with nix and native packaging/scripts
    x86_64-linux = {
      arch = "amd64";
      sha256 = "0vraf6iwbddpcy4l9msks6lmi35k7wfgpafikb56k3qinvvcjm9b";
    };
    aarch64-linux = {
      arch = "arm64";
      sha256 = "sha256-TBbDCKlGb+n5V/IqWPuZBef3dZ//Pt7v3gdOvyoOrjU=";
    };
    aarch64-darwin = {
      arch = "arm64";
      sha256 = "sha256-TBbDCKlGb+n5V/IqWPuZBef3dZ//Pt7v3gdOvyoOrjU=";

    };
    x86_64-darwin = {
      arch = "amd64";
      sha256 = "0vraf6iwbddpcy4l9msks6lmi35k7wfgpafikb56k3qinvvcjm9b";
    };
  };
in if builtins.hasAttr system base-images then
  base-images.${system}
else
  base-images.x86_64-linux
