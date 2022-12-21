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
    mkdir -p $out

    ${pkgs.lib.meta.getExe packages.centauri-codegen} \
      --path $out \
      --parachain-wasm=${packages.dali-runtime}/lib/runtime.optimized.wasm \
      --relaychain-wasm=${packages.rococo-wasm-runtime}/lib/rococo_runtime.compact.compressed.wasm

    rustfmt --edition=2018 $out/*
  '';
}
