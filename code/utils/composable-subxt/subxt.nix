{pkgs} :
pkgs.rustPlatform.buildRustPackage rec {
  pname = "subxt";
  version = "0.24.0";

  src = pkgs.fetchCrate {
    inherit pname version;
    sha256 = "sha256-BhDHJIBshbgJiU5nVffjeXwOLMhUG0F9o8jsg1oSQrw=";
  };

  cargoHash = "sha256-JmBZcDVYJaK1cK05cxx5BrnGWp4t8ca6FLUbvIot67s=";
  cargoDepsName = pname;
}