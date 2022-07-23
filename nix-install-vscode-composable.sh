# install nix under `vscode` user targeting composable cache
echo "Installin via $1 and using $2"

# so we avoid using symbols which may not execute well in shells
# easy to cat what is going on
curl --location $1 > ./nix-install.sh
chmod +x ./nix-install.sh 
./nix-install.sh

# force nix upon user
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bashrc
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.profile
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bash_profile
chmod +x ~/.nix-profile/etc/profile.d/nix.sh
~/.nix-profile/etc/profile.d/nix.sh
cat ~/.nix-profile/etc/profile.d/nix.sh

ls ~/.nix-profile/bin
# WTF? why it does not work?
export PATH="~/.nix-profile/bin:$PATH"

(
    cd ~/.nix-profile/bin
    # ensure user is on same binaries we are
    ./nix-channel --add $2 nixpkgs
    ./nix-channel --update                
    ./nix-env --install --attr nixpkgs.cachix
    find / -name cachix
    cachix use composable-community       
)