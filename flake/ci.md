# Actions runner setup steps

1. `installimage` `ubuntu-22.04`
2. `adduser actions-runner`
3. `passwd --delete actions-runner` 
4. `curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install --no-confirm`
5. `source /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh`
6. `su actions-runner && cd  /home/actions-runner/`
7. follow install guide from github using defaults and name `hetzner-ax161-135.181.17.107` and label `x86_64-linux-32C-128GB-2TB`

8. 
```bash
cat >> /etc/nix/nix.conf << EOF
    sandbox = relaxed
    narinfo-cache-negative-ttl = 0      
    system-features = kvm     
    trusted-users = root actions-runner
    keep-derivations = true
    keep-outputs = true
EOF
```

8. `/home/actions-runner/actions-runner/svc.sh install actions-runner && /home/actions-runner/actions-runner/svc.sh start`.

9. run `cachix as service`

11. `nix-channel --add https://nixos.org/channels/nixos-unstable nixpkgs && nix-channel --update`

10. `nix profile install nixpkgs#git nixpkgs#git-lfs nixpkgs#docker` 

11.  Sure do not do this in production. Solution is to nixos-generators custom image with public ssh and github runner built in and using nix rebuild to update config (or can use home-manager on ubuntu). 