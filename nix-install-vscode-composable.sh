# install nix under `whoami` user targeting composable cache
echo "Installing via script at $1  and using $2 channel"

# so we avoid using symbols which may not execute well in shells
# easy to cat what is going on
curl --location $1 > ./nix-install.sh
chmod +x ./nix-install.sh 
./nix-install.sh
chmod +x ~/.nix-profile/bin

echo "Force nix upon user"
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bashrc
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.profile
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bash_profile
export PATH="/home/$(whoami)/.nix-profile/bin:$PATH"

chmod +x ~/.nix-profile/etc/profile.d/nix.sh
~/.nix-profile/etc/profile.d/nix.sh

# NOTE: for some reason installed stuff is not executable...
chmod +x ~/.nix-profile/bin/nix-channel
chmod +x ~/.nix-profile/bin/nix-env

echo "Ensure user is on same binaries we are"
nix-channel --add $2 nixpkgs
nix-channel --update                
nix-env --install --attr nixpkgs.cachix
chmod +x ~/.nix-profile/bin/cachix

echo "Cachix"
cachix use composable-community       
