{ pkgs, packages }:
pkgs.stdenv.mkDerivation {
  name = "dali-subxt-client";
  dontUnpack = true;
  buildInputs = [
    packages.centauri-codegen
    packages.dali-runtime
    packages.rococo-wasm-runtime
    pkgs.rustfmt
  ];
  installPhase = ''
    mkdir -p $out/tmp

    ${pkgs.lib.meta.getExe packages.centauri-codegen} \
      --path $out/tmp \
      --parachain-wasm=${packages.dali-runtime}/lib/runtime.optimized.wasm \
      --relaychain-wasm=${packages.rococo-wasm-runtime}/lib/rococo_runtime.compact.compressed.wasm

    for file in $(ls $out/tmp); do
      cat $out/tmp/$file | rustfmt --edition=2018 --emit=stdout > $out/$file
    done
    rm -r $out/tmp
  '';
}
