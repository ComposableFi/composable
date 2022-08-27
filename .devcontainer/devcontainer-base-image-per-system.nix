system:
let
  base-images = {
    x86_64-linux = {
      arch = "amd64";
      imageDigest = "sha256:61d1944c51e3b8bbbe3e08bf2fb580cb620951d7d4f6b01fdb3697ef40e4802b";
      sha256 = "0lxdncasmrh07mcgg2nrkmzd6m3slbic36w9x0cxppii2496cq8q";
    };
    x86_64-darwin = {
      arch = "amd64";
      imageDigest = "sha256:61d1944c51e3b8bbbe3e08bf2fb580cb620951d7d4f6b01fdb3697ef40e4802b";
      sha256 = "0lxdncasmrh07mcgg2nrkmzd6m3slbic36w9x0cxppii2496cq8q";
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
