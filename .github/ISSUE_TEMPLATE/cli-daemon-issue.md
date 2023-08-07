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
9. `df --human-readable /tmp/`
10. `ps -A x u`
11. Exact command used to run.
12. Tool version.
13. Full output of command of run.
14. `Output files` of run.
15. Human description of expected and observed behaviour

## Logs

Run tool with logs enabled, for example like it is documented for Nix.

##  Output files

Local running network usually located in `/tmp/`, specifically in `/tmp/composable-devnet/` directory. 
Also files may be in working directory, Nix in `result`. 
