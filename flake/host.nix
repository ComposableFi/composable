{ pkgs, ... }: {
  services.openssh = {
    enable = true;
    settings.PasswordAuthentication = false;
    settings.KbdInteractiveAuthentication = false;
    settings.PermitRootLogin = "yes";
  };
  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQDNY+BfeToEN1+1HTSggNrFHYhYFl9H9dPgIJy558OgWHsYrhMA7PHUy3VK0DjnIT9jFU1PF3/v1tpgUij9bOm6Md6N7Dn2/XL6/FqPNJ9i408V6DdCmH65aJ2tnSJJ4aicD9P39MHVG6tYPKJX9BrHiGzLPLi+c/4CWXIcj/u4aAuvspfCu6a5jWPj03XBwUUbkmdgyvEJ7wJoiOKE1b/Ilxiithau7w0GgHG3e1RUMeVy4aaNET3sTlhiJf4k+cL+7MIM13wUiqjglyzBfMGQKPsaHFuMMsfK4lHploLkBZeopiIxyRzQeRODFsuUSR+J/oL7TiIyMALCEqErRb8OrmPI7NKYRqokfU20YTgOSW+t7JxCx5vtYHyw2HVMZTnSeHAFfcclBh1Vi4vqHymNhJXEh35k/iLdUNdcMgHyqmjZZecpAT3fIULOlGfyfc6kKFmfAYWFcci+ByE0e0T82BlLWJHBuQTByu2w+IzUA81uKBqBqNgLayi49Bpwg5k= dz@pop-os"
    "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQCXsAg4FHGlI7YNSM86z+nLpoLKEP8pln4HoqP7GcCYYScoq7OduiYw2uvedYJU2jA91i+Ep6l+mbzh+qxMkAyte80bIHeQbo5f47JXUJblKrveGaVb3mPKQJ7MYVgvw+WySwZcqQrEKbTo+bp6DAIYrCBvWkIdFss//DDMGbcyX3oF5gqZ5DJsiD4q89chY1uOtwIWdjLHe+9LMud7/OetRWwHpbk5i3BFPa3hQiixu48/TynPlzMk4tYSXnmelhzybtrI0j40/UQnpb8nHj/U0+ZAKG2OcNo+wBRuYWUdkoGZh3p5diy7oaPap+AmA+d25qrBnB9Xr2t1QIUoSP7I4+Bq7S4AVKLPQhU1vo8mA4TWzhuHQ4tz5f3iN5XOMKxoxG7wMgL5Y3xF60sqXWlsNJ4VDb285WxuFvEhtPzsRHtdkPMHLjDxePpwASxTkTgoha1FCapvuQOjE73B66urmB5vhpVE5MYb9S5tALEwX95oUJhscwjCcr+9oZFOn5k= dz@pop-os"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIM1+8YaFQCOY4D52kpPs8sgsDdfHFqjHIdWopedt4P7x sre@composable.finance"
  ];
}
