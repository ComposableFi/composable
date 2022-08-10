system:
let
  base-images = {
    x86_64-linux = {
      arch = "amd64";
      sha256 = "65fde405acf4a8e9c228c62ebd677b3a817669c5d759ef2c8a95b0b7069285d3";
      imageDigest = "sha256:65fde405acf4a8e9c228c62ebd677b3a817669c5d759ef2c8a95b0b7069285d3";
    };
    x86_64-darwin = {
      arch = "amd64";
      sha256 = "65fde405acf4a8e9c228c62ebd677b3a817669c5d759ef2c8a95b0b7069285d3";
      imageDigest = "sha256:65fde405acf4a8e9c228c62ebd677b3a817669c5d759ef2c8a95b0b7069285d3";
    };
    aarch64-linux = {
      arch = "arm64";
      sha256 = "c4425136657851e389eb5091bc8c726aa48c84bbebc493aa530a869209aefab0";
      imageDigest = "sha256:c4425136657851e389eb5091bc8c726aa48c84bbebc493aa530a869209aefab0";
    };
    aarch64-darwin = {
      arch = "arm64";
      sha256 = "c4425136657851e389eb5091bc8c726aa48c84bbebc493aa530a869209aefab0";
      imageDigest = "sha256:c4425136657851e389eb5091bc8c726aa48c84bbebc493aa530a869209aefab0";
    };
  };
in if builtins.hasAttr system base-images then
  base-images.${system}
else
  base-images.x86_64-linux
