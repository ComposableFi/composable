{ lib, ... }: {
  networking = {
    nameservers = [ "8.8.8.8" ];
    defaultGateway = "172.31.1.1";
    defaultGateway6 = {
      address = "fe80::1";
      interface = "eth0";
    };
    dhcpcd.enable = false;
    usePredictableInterfaceNames = lib.mkForce false;
    interfaces = {
      eth0 = {
        ipv4.addresses = [{
          address = "65.109.137.28";
          prefixLength = 32;
        }];
        ipv6.addresses = [
          {
            address = "2a01:4f9:c012:9c10::1";
            prefixLength = 64;
          }
          {
            address = "fe80::9400:2ff:fe8d:1e76";
            prefixLength = 64;
          }
        ];
        ipv4.routes = [{
          address = "172.31.1.1";
          prefixLength = 32;
        }];
        ipv6.routes = [{
          address = "fe80::1";
          prefixLength = 128;
        }];
      };

    };
  };
  services.udev.extraRules = ''
    ATTR{address}=="96:00:02:8d:1e:76", NAME="eth0"

  '';
}
