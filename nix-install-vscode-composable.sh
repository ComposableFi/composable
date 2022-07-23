# install nix under `vscode` user targeting composable cache
echo $1
curl --location $1 > ./nix-install.sh
chmod +x ./nix-install.sh
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bashrc
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.profile

echo $2
# nix-channel --add ${{env.NIX_NIXPKGS_CHANNEL}} nixpkgs
# nix-channel --update                
# nix-env --install --attr nixpkgs.cachix
# cachix use composable-community       