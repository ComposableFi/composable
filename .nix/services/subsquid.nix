{pkgs, packages, ...}:
{
  image.contents = [ pkgs.bash pkgs.coreutils ];
}
