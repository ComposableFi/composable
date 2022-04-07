ubuntu@tokyo-subscan-dalinode:~$ cat /etc/systemd/system/dali-subscan-node.service

ExecStart=/usr/bin/composable \
--name subscan-node-dali \
--unsafe-ws-external \
--ws-port 9844 \
--rpc-cors=all \
--pruning=archive \
--chain=composable-westend \
--base-path /var/lib/composable-data/ \
--listen-addr=/ip4/0.0.0.0/tcp/30334 \
--execution=wasm \
-- --execution=wasm \
--listen-addr=/ip4/0.0.0.0/tcp/30333 \
--pruning=archive \
--sync Full
#--public-addr=/ip4/34.79.44.160/tcp/30333

# (file size)
LimitFSIZE=infinity
# (cpu time)
LimitCPU=infinity
# (virtual memory size)
LimitAS=infinity
# (locked-in-memory size)
LimitMEMLOCK=infinity
# (open files)
LimitNOFILE=64000
# (processes/threads)
LimitNPROC=64000

Restart=always
RestartSec=120

[Install]
WantedBy=multi-user.target
