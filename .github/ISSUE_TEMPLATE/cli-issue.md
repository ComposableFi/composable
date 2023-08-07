# Issue during usage command line tool and daemons

In order to report issue during usage of command line tools and daemons,
please attach outputo of:

1. `uname -a`
2. `nix version`
3. `nix show-config`
4. `docker version`
5. `git version`
6. `git log -1`
7. `cat /proc/meminfo`
8. `cat /proc/cpuinfo`
9. Exact command used to run.
10. Full output of command of run.
12. `Output files` of run.
13. Human description of expected and observed behaviour

##  Output files

Local running network usually located in `/tmp/`, specifically in `/tmp/composable-devnet/` directory. 
Also files may be in working directory, Nix in `result`. 
