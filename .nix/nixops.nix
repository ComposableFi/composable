{ pkgs }:
(pkgs.nixopsUnstable.override {
  overrides = (self: super: {
    # FIXME: probably useless once 2.0 is stable
    # NOTE: must use because 1.7 does not work for us
    nixops = super.nixops.overridePythonAttrs (_: {
      src = pkgs.fetchgit {
        url = "https://github.com/NixOS/nixops";
        rev = "35ac02085169bc2372834d6be6cf4c1bdf820d09";
        sha256 = "1jh0jrxyywjqhac2dvpj7r7isjv68ynbg7g6f6rj55raxcqc7r3j";
      };
    });
  });
})
