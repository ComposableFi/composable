# install nix under `vscode` user targeting composable cache
echo "Installin via $1 and using $2"

# so we avoid using symbols which may not execute well in shells
# easy to cat what is going on
curl --location $1 > ./nix-install.sh
chmod +x ./nix-install.sh 
./nix-install.sh
chmod +x ~/.nix-profile/bin

# force nix upon user
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bashrc
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.profile
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bash_profile
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bash_login
chmod +x ~/.nix-profile/etc/profile.d/nix.sh
~/.nix-profile/etc/profile.d/nix.sh

# NOTE: for some reason installed stuff is not executable...
chmod +x ~/.nix-profile/bin/nix-channel
chmod +x ~/.nix-profile/bin/nix-env

export PATH="/home/vscode/.nix-profile/bin:$PATH"

echo "Ensure user is on same binaries we are"
nix-channel --add $2 nixpkgs
nix-channel --update                
nix-env --install --attr nixpkgs.cachix
chmod +x ~/.nix-profile/bin/cachix

echo "Cachix"
cachix use composable-community       
