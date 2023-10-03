---
name: CLI/daemon issue
about: Issue during usage command line tool and daemons
title: "Issue during usage command line tool and daemons"
labels: issue
assignees: ""
---

# Description

Supported are NixOs and Debian derived Linuxes on aarch64 and x86_64 architectures, uncluding running these in OCI containers. 

In order to report issue during usage of command line tools and daemons,
please attach output of next commands as run from working directory you run command for:

1. `uname -a`
2. `nix version`
3. `nix show-config`
4. `docker version`
5. `git version`
6. `git log -1`
7. `cat /proc/meminfo`
8. `cat /proc/cpuinfo`
9. `stat  /tmp && df --human-readable /tmp/`
10. `stat  /nix && df --human-readable /nix/`
11. `ps -A x u`
13. `netstat --tcp --udp --listening --numeric --programs  | grep LISTEN`
14. Exact command used to run.
15. Tool version.
16. Full output of command of run.
17. `Output files` of run.
18. Human description of expected and observed behaviour

## Logs

Run tool with logs enabled, for example like it is documented for Nix.

##  Output files

Local running network usually located in `/tmp/`, specifically in `/tmp/composable-devnet/` directory and `/tmp/process-compose*` files. 
Also files may be in working directory, Nix in `result`. 
