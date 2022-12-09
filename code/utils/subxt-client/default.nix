{ pkgs, subxt }:
let
  registriesConf = pkgs.writeText "registries.conf" ''
    [registries.search]
    registries = ['docker.io']
    [registries.block]
    registries = []
  '';

  containersConf = pkgs.writeText "containers.conf" ''
    [containers]
    default_capabilities = [
        "CHOWN",
        "DAC_OVERRIDE",
        "FOWNER",
        "FSETID",
        "KILL",
        "NET_BIND_SERVICE",
        "SETFCAP",
        "SETGID",
        "SETPCAP",
        "SETUID",
        "SYS_CHROOT"
    ]
    default_sysctls = [
        "net.ipv4.ping_group_range=0 0",
    ]
    [secrets]
    [secrets.opts]
    [network]
    [engine]
    [engine.runtimes]
    [engine.volume_plugins]
    [machine]
  '';

  tools = pkgs.writeText "tools.sh" ''
    wait_for_ws_server_to_be_ready() {
        HOST=$1
        PORT=$2
        set +e
        while true; do
            echo "Trying to connect to $HOST:$PORT..."
            echo '{"id":1, "jsonrpc":"2.0", "method": "system_version"}' | websocat --oneshot ws://$HOST:$PORT > /dev/null
            exit_code=$?
            if [ $exit_code -eq 0 ]; then
                break
            fi
            sleep 1
        done
        set -e
    }
  '';

in pkgs.stdenv.mkDerivation {
  name = "subxt-client";
  dontUnpack = true;
  buildInputs = [
    pkgs.podman
    pkgs.runc # Container runtime
    pkgs.conmon # Container runtime monitor
    pkgs.skopeo # Interact with container registry
    pkgs.slirp4netns # User-mode networking for unprivileged namespaces
    pkgs.fuse-overlayfs # CoW for images, much faster than default vfs
    pkgs.websocat
    pkgs.rustfmt
    subxt
  ];
  installPhase = ''
    mkdir $out
    export HOME=$(pwd)
    mkdir --parent $out/etc/containers $HOME/.config/containers

    ln -s ${pkgs.skopeo.src}/default-policy.json $out/etc/containers/policy.json
    ln -s ${registriesConf} $out/etc/containers/registries.conf
    ln -s ${containersConf} $HOME/.config/containers/containers.conf
    source ${tools}

    podman version

    container_id=$(podman run --rm -d -u$(id -u):$(id -g) -p9944:9944 -p9988:9988 docker.io/composablefi/composable-sandbox:latest)
    podman ps
    wait_for_ws_server_to_be_ready 127.0.0.1 9944
    wait_for_ws_server_to_be_ready 127.0.0.1 9988

    ${subxt}/bin/subxt codegen --url ws://localhost:9944 | rustfmt --edition=2018 --emit=stdout > $out/rococo_subxt.rs
    ${subxt}/bin/subxt codegen --url ws://localhost:9988 | rustfmt --edition=2018 --emit=stdout > $out/composable_subxt.rs

    podman logs --tail 10 $container_id
    podman stop $container_id
    podman system prune --all --force
    rm -r $out/etc
  '';
  # podman requires access to cgroups (/sys/fs/cgroup)
  __noChroot = true;
}
