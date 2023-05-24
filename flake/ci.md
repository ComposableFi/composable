# Actions runner setup steps

1. `installimage` `ubunutu-22.04`
2. `adduser actions-runner` 
3. `curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install --no-confirm`
4. `source /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh`
5. `su actions-runner && cd  /home/actions-runner/`
6. follow install guide from github using defaults and name `hetzner-ax161-135.181.17.107` and label `x86_64-linux-32C-128GB-2TB`
7. add service


```bash
cat > /etc/systemd/system/actions-runner.service << EOF
[Unit]
Description=github-runner

[Service]
Type=simple
ExecStart=/home/actions-runner/actions-runner/run.sh
Restart=on-failure
User=actions-runner
WorkingDirectory=/home/actions-runner/actions-runner/
RuntimeDirectory=/home/actions-runner/actions-runner/
[Install]
WantedBy=multi-user.target
EOF
```

7. `systemctl daemon-reload && systemctl start actions-runner.service && systemctl status actions-runner.service` 

8. Sure do not do this in production. Solution is to nixos-generators custom image with public ssh and github runner built in and using nix rebuild to update config. 