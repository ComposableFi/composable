{ pkgs, ... }: {
  environment.systemPackages = with pkgs; [ helix ];
  services.openssh = {
    enable = true;
    settings.PasswordAuthentication = false;
    settings.KbdInteractiveAuthentication = false;
    settings.PermitRootLogin = "yes";
  };
  users.users."root".openssh.authorizedKeys.keys = [
    "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQCXsAg4FHGlI7YNSM86z+nLpoLKEP8pln4HoqP7GcCYYScoq7OduiYw2uvedYJU2jA91i+Ep6l+mbzh+qxMkAyte80bIHeQbo5f47JXUJblKrveGaVb3mPKQJ7MYVgvw+WySwZcqQrEKbTo+bp6DAIYrCBvWkIdFss//DDMGbcyX3oF5gqZ5DJsiD4q89chY1uOtwIWdjLHe+9LMud7/OetRWwHpbk5i3BFPa3hQiixu48/TynPlzMk4tYSXnmelhzybtrI0j40/UQnpb8nHj/U0+ZAKG2OcNo+wBRuYWUdkoGZh3p5diy7oaPap+AmA+d25qrBnB9Xr2t1QIUoSP7I4+Bq7S4AVKLPQhU1vo8mA4TWzhuHQ4tz5f3iN5XOMKxoxG7wMgL5Y3xF60sqXWlsNJ4VDb285WxuFvEhtPzsRHtdkPMHLjDxePpwASxTkTgoha1FCapvuQOjE73B66urmB5vhpVE5MYb9S5tALEwX95oUJhscwjCcr+9oZFOn5k= dz@pop-os"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIM1+8YaFQCOY4D52kpPs8sgsDdfHFqjHIdWopedt4P7x sre@composable.finance"
  ];
}
