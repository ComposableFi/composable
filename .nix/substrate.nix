{ pkgs, rust-overlay }:
with pkgs; {
  __noChroot = true;
  doCheck = false;
  buildInputs = [ openssl zstd ];
  nativeBuildInputs = [ clang git rust-overlay ];
  # TODO: moved these to some `cumulus based derivation`
  LD_LIBRARY_PATH =
    lib.strings.makeLibraryPath [ stdenv.cc.cc.lib llvmPackages.libclang.lib ];
  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
}
