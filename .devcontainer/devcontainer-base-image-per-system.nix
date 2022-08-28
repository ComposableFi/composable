# system:
# let
#   base-images = {
#     x86_64-linux = {
#       arch = "amd64";
#       imageDigest = "sha256:61d1944c51e3b8bbbe3e08bf2fb580cb620951d7d4f6b01fdb3697ef40e4802b";
#       sha256 = "0lxdncasmrh07mcgg2nrkmzd6m3slbic36w9x0cxppii2496cq8q";
#     };
#     x86_64-darwin = {
#       arch = "amd64";
#       imageDigest = "sha256:61d1944c51e3b8bbbe3e08bf2fb580cb620951d7d4f6b01fdb3697ef40e4802b";
#       sha256 = "0lxdncasmrh07mcgg2nrkmzd6m3slbic36w9x0cxppii2496cq8q";
#     };
#     aarch64-linux = {
#       arch = "arm64";
#       imageDigest = "sha256:61d1944c51e3b8bbbe3e08bf2fb580cb620951d7d4f6b01fdb3697ef40e4802b";
#       sha256 = "1ph24rfdnbl3xm2qrqassh8bwjnd3d5ryq5sdggcdnq3chx81ayj";
#     };
#     aarch64-darwin = {
#       arch = "arm64";
#       imageDigest = "sha256:61d1944c51e3b8bbbe3e08bf2fb580cb620951d7d4f6b01fdb3697ef40e4802b";
#       sha256 = "1ph24rfdnbl3xm2qrqassh8bwjnd3d5ryq5sdggcdnq3chx81ayj";
#     };
#   };
# in
# if builtins.hasAttr system base-images then
#   base-images.${system}
# else
#   base-images.x86_64-linux
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
