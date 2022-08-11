
# https://github.com/microsoft/vscode-dev-containers/tree/main/containers/debian
# https://github.com/numtide/flake-utils/blob/master/default.nix       
system:
let supported-nix-to-container-images = { 
    # NOTE: we peeky container version to make these work version compatible with nix and native packaging/scripts
        x86_64-linux ={
            arch = "amd64";
        };
        aarch64-linux ={
            arch = "arm64";
        };
        aarch64-darwin ={
            arch = "arm64";
        };
        x86_64-darwin ={
            arch = "amd64";
        };
    };
in 
  if builtins.hasAttr system supported-nix-to-container-images then 
    supported-nix-to-container-images.${system}
  else supported-nix-to-container-images.x86_64-linux