## Guide

1. Use dedicated non cloud hardware for heavy jobs (32 CPUs) for Linux
2. Use dedicated non cloud hardware for Mac jobs
3. Use one to one hardware as used in production for benchmarks
4. Use Nix native services (Hercules, Nixbuild, Hydra) for immutable pure nix builds (they are fastest for Nix and with best debugging experience)
5. Use default GH runners for super light jobs (2 CPU cores).
6. Use BlueJet or GH Larger runners for light jobs (4-8 CPU cores).
7. Observer jobs via dashboards (Trunk) and optimize

# Actions runner setup steps

1. `installimage -i images/Ubuntu-2204-jammy-amd64-base.tar.gz -G yes -a -n hetzner-ax161-{N}`
2. `adduser actions-runner && passwd --delete actions-runner` 
3. `curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install --no-confirm && source /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh && nix-channel --add https://nixos.org/channels/nixos-unstable nixpkgs && nix-channel --update && nix profile install nixpkgs#git nixpkgs#git-lfs nixpkgs#docker`
3. 
```bash
cat >> /etc/nix/nix.conf << EOF
    sandbox = relaxed
    narinfo-cache-negative-ttl = 0      
    system-features = kvm     
    trusted-users = root actions-runner
EOF
```

0. `apt install qemu-system-x86 --yes`
1. `su actions-runner && cd /home/actions-runner/`
2. follow install guide from github using defaults and name `hetzner-ax161-{N}` and label `x86_64-linux-32C-128GB-2TB`


1. `cd /home/actions-runner/actions-runner && ./svc.sh install actions-runner && ./svc.sh start && systemctl daemon-reload`
 
2. `usermod --append --groups kvm actions-runner && chmod 666 /dev/kvm`
3.   Sure do not do this in production. Solution is to nixos-generators custom image with public ssh and github runner built in and using nix rebuild to update config (or can use home-manager on ubuntu). 