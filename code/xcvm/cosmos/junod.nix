{ buildGoModule, rustPlatform, fetchFromGitHub, patchelf, system, lib }:
let
  libwasmvm = rustPlatform.buildRustPackage {
    name = "libwasmvm";
    src = fetchFromGitHub {
      owner = "CosmWasm";
      repo = "wasmvm";
      rev = "1afba37bfd0eda626d11ec760f51b16cb4254167";
      sha256 = "sha256-/wS+kZFu4RTO7Ump21dM9DpQBxTQ87BlCblE0JYMdiY=";
    };
    cargoHash = "sha256-WMfYsGtzOCxbhyoRRLtHg9H8ckPCByjsBSZCXimj/80=";
    sourceRoot = "source/libwasmvm";
    doCheck = false;
  };
in buildGoModule {
  name = "junod";
  doCheck = false;
  src = fetchFromGitHub {
    owner = "CosmosContracts";
    repo = "juno";
    rev = "e6f9629538a88edf11aa7e7ed3d68c61f8e96aa6";
    sha256 = "sha256-ro4ACIolNPbGnZnK610uX1KPO+b728O284PlKrPY1JY=";
  };
  vendorSha256 = "sha256-yGvxHS3wzjY1ZPUwuLK6B1+Xii8ipzhJpGi2Gl5Ytdo=";
  fixupPhase = ''
    ${patchelf}/bin/patchelf \
      --shrink-rpath \
      --allowed-rpath-prefixes /nix/store \
      --replace-needed libwasmvm.${
        builtins.head (lib.strings.split "-" system)
      }.so libwasmvm.so \
      $out/bin/junod
    ${patchelf}/bin/patchelf \
      --add-rpath ${libwasmvm}/lib \
      $out/bin/junod
  '';
}
