# Actions runner setup steps

1. ssh 
2. installimage ubunutu 22.04
3. ssh again
4. install nix
5. adduser `actions-runner`  
6. follow install guid from github using defaults

7. add service

```
/etc/systemd/system/actions-runner.service < EOF

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

7. systemctl daemon-relaod && start && status 
