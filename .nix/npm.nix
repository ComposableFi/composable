{ pkgs }: {
  nativeBuildInputs = with pkgs; [
    yarn
    nodejs
    python3
    pkg-config
    vips
    python3
    nodePackages.node-gyp-build
    nodePackages.node-gyp
    nodePackages.typescript
    coreutils
  ];
  containerRunInputs = [ pkgs.coreutils pkgs.glibc.bin pkgs.bash pkgs.procps ];
}
