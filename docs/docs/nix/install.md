# Install Nix

## Installing Nix

Once you have determined which one you want to use, [follow the official Nix
installation instructions](https://zero-to-nix.com/start/install).



## Configuring your Nix install

### On NixOS

In your Nix system config (`/etc/nixos/configuration.nix` by default), configure `nix` like this:

```nix
{
  nix = {
    useSandbox = "relaxed";
    extraOptions = ''
      experimental-features = nix-command flakes
    '';
    sandbox = "relaxed";
  };
}
```

### On non-NixOS

Set the contents of `~/.config/nix/nix.conf` to this:

```nix
experimental-features = nix-command flakes
sandbox = relaxed
```

### Using flags

If you cannot edit these config files, then you can pass the following flags to `nix`. 

```shell
--extra-experimental-features nix-command --extra-experimental-features flakes --no-sandbox --options sandbox relaxed
```

---

You are now ready to start [running packages](./run-packages)!
