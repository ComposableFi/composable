# build arm64 container via nix and load it into local docker
docker run --name "composable-container-builder" -it -v ${PWD}:/composable nixos/nix bash -c "cd composable && nix build .#packages.aarch64-linux.codespace-container --extra-experimental-features nix-command --extra-experimental-features flakes" && \
docker cp -L composable-container-builder:/composable/result ./devcontainer.tar.gz && \
docker rm composable-container-builder && \
docker load --input ./devcontainer.tar.gz
