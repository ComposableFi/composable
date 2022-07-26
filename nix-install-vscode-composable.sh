# install nix under `whoami` user targeting composable cache
#
# works:
# - ci runners
# - dev containers
#
# does not work:
# - on nixos
# - if there is no home folder of current user
# - under root

set -o errexit -o pipefail

url=${1:-https://releases.nixos.org/nix/nix-2.10.3/install}
channel=${2:-https://nixos.org/channels/nixpkgs-22.05-darwin}
cachix=${3:-composable-community}

echo "Installing via script at $url  and using $channel channel"

# so we avoid using symbols which may not execute well in shells
# easy to cat what is going on
curl --location $url > ./nix-install.sh
chmod +x ./nix-install.sh 
mkdir --mode=0755 /nix && chown $(whoami) /nix
./nix-install.sh --no-daemon
rm ./nix-install.sh
echo "ensure nix can be executed if it is not"
chmod +x ~/.nix-profile/bin/nix-channel
chmod +x ~/.nix-profile/bin/nix-env
chmod +x ~/.nix-profile/bin/nix

echo "Force nix upon user"
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bashrc
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.profile
echo "source ~/.nix-profile/etc/profile.d/nix.sh" >> ~/.bash_profile
export PATH="/home/$(whoami)/.nix-profile/bin:$PATH"

chmod +x ~/.nix-profile/etc/profile.d/nix.sh
~/.nix-profile/etc/profile.d/nix.sh

echo "Flakes and commands support"
# not using global /etc/nix/nix.conf
mkdir --parents /home/$(whoami)/.config/nix/
echo "experimental-features = nix-command flakes" > /home/$(whoami)/.config/nix/nix.conf

echo "Ensure user is on same binaries we are"
nix-channel --add $channel nixpkgs && nix-channel --update                
nix-env --install --attr nixpkgs.cachix
chmod +x ~/.nix-profile/bin/cachix

echo "Cachix"
cachix use $cachix       
