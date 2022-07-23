# install nix under `vscode` user targeting composable cache
echo "Installin via $1 and using $2"

curl --location $1 > ./nix-install.sh
chmod +x ./nix-install.sh
./nix-install.sh
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bashrc
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.profile
chmod +x ~/.nix-profile/etc/profile.d/nix.sh
~/.nix-profile/etc/profile.d/nix.sh

nix-channel --add $2 nixpkgs
nix-channel --update                
nix-env --install --attr nixpkgs.cachix
cachix use composable-community       