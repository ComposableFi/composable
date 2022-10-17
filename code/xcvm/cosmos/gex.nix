{ buildGoModule, fetchFromGitHub }:
buildGoModule {
  name = "gex";
  doCheck = false;
  src = fetchFromGitHub {
    owner = "cosmos";
    repo = "gex";
    rev = "bc168741b2019745d343606d31b5c274f216fc3f";
    sha256 = "sha256-7jtCpOTHamXAInfKYkMIDFKF4lViuPkusThj4ggGUbg=";
  };
  vendorSha256 = "sha256-3vD0ge0zWSnGoeh5FAFEw60a7q5/YWgDsGjjgibBBNI=";
}
