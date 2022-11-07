# Frequently Occurring Issues

This page is supposed to help you if you have problems running our Nix setup.


## `LLVM ERROR: IO failure on output stream: No space left on device`

If you get an `No space left on device` error message even though enough disk space is available,
you will need to increase the maximum size of your `/tmp` directory.

You can achieve this using 2 ways.
### 1. Temporary fix

To fix the issue temporarily please use the following command as `root` user.
```sh
$ mount -o remount,size=16G /tmp/
```


### 2. Permanent fix

To fix this issue permanently, you'll need to edit the `/etc/fstab` file, using a text editor of your choice.

Look for a line starting with `tmpfs` and update it like the following, 
or append it to the bottom if it doesn't exist yet.
```
tmpfs /tmp tmpfs rw,nodev,nosuid,size=16G 0 0
```


## `Too many open files`

If you get an error like `Too many open files`, you'll need to increase this specific limit.

The most easy way is to bundle the start command with the `ulimit` command. Like the following.

```sh
$ bash -c 'ulimit -n 10000; nix run github:composablefi/composable#devnet-dali`
```


## experimental Nix feature is disabled
## or `derivation has '__noChroot' set, but that';s not allowed when 'sandbox' is 'true'
## or Nix run command does not respect `nix.conf` configuration.

If you get any of the above errors, you may need to put your Nix configuration into `/etc/nix/nix.conf`,
instead of `~/.config/nix/nix.conf`, which was described during the installation process.


## Can't re-install Nix after macOS update

To successfully re-install Nix after an macOS update, you need to remove the existing `/nix` directory,
and start the re- installer again.

```sh
$ sudo rm -rf /nix 
```
