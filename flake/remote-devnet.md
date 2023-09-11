
1. boot to rescue
2. add CI key to echo .ssh/authorized
3. boot into kexec nix 
```
curl -L https://github.com/nix-community/nixos-images/releases/download/nixos-unstable/nixos-kexec-installer-noninteractive-x86_64-linux.tar.gz | tar -xzf- -C /root
/root/kexec/run
```

aF8wvg3RH8uuWs

adduser actions-runner --disabled-password && passwd --delete actions-runner && mkdir -m 0755 /nix && chown actions-runner /nix


sh <(curl -L https://nixos.org/nix/install) --no-daemon && . /home/actions-runner/.nix-profile/etc/profile.d/nix.sh

cat > /etc/nix/nix.conf << EOF
    accept-flake-config = false
    allow-import-from-derivation = true
    allowed-users = root actions-runner
    auto-optimise-store = true
    connect-timeout = 30
    cores = 64
    download-attempts = 5
    experimental-features = nix-command flakes cgroups
    gc-reserved-space = 100000000000
    http-connections = 32
    http2 = true
    keep-derivations = true
    keep-failed = false
    keep-outputs = true
    max-free = 1000000000000
    max-jobs = 1
    min-free = 200000000000
    narinfo-cache-negative-ttl = 0
    require-sigs = true
    sandbox = relaxed
    sandbox-fallback = false
    stalled-download-timeout = 300
    substitute = true
    substituters = https://nix-community.cachix.org/ https://cache.nixos.org/ https://composable.cachix.org/ https://devenv.cachix.org/ https://cosmos.cachix.org https://nixpkgs-update.cachix.org
    system-features = kvm     
    timeout = 3600
    trusted-public-keys = nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs= cosmos.cachix.org-1:T5U9yg6u2kM48qAOXHO/ayhO8IWFnv0LOhNcq0yKuR8= cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY= composable.cachix.org-1:J2TVJKH4U8xqYdN/0SpauoAxLuDYeheJtv22Vn3Hav8= nixpkgs-update.cachix.org-1:6y6Z2JdoL3APdu6/+Iy8eZX2ajf09e4EE9SnxSML1W8= devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=
    trusted-substituters = https://nix-community.cachix.org/ https://cache.nixos.org/ https://composable.cachix.org/ https://devenv.cachix.org/ https://cosmos.cachix.org https://nixpkgs-update.cachix.org
    trusted-users = root actions-runner
    use-cgroups = true
    warn-dirty = true
EOF