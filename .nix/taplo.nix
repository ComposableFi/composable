{ pkgs, crane-stable }:
crane-stable.buildPackage {
  src = pkgs.fetchFromGitHub {
    owner = "tamasfe";
    repo = "taplo";
    rev = "eeb62dcbada89f13de73cfc063ffe67a890c4bc6";
    hash = "sha256-ggcyOsA4cyo5l87cZmOMI0w1gCzmWy9NRJiWxjBdB1E=";
  };
}
