{ pkgs, composable}:
pkgs.stdenv.mkDerivation rec {
    name = "composable-${composable.name}-${composable.version}";
    version = composable.version;
    src = pkgs.fetchurl {
      # TODO: remove - use cachix for builds - or pure buildsfrom repo
      url = "https://storage.googleapis.com/composable-binaries/community-releases/${composable.name}/${name}.tar.gz";
      sha256 = composable.hash;
    };
    nativeBuildInputs = [
      pkgs.autoPatchelfHook
    ];
    autoPatchelfIgnoreMissingDeps = [ "*" ]; 
    buildInputs = [ pkgs.stdenv.cc.cc pkgs.zlib pkgs.rocksdb ];
    installPhase = ''
      tar -xvf $src
      mkdir -p $out/bin
      mv release/composable $out/bin
      mv doc $out
    '';
    ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
    LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath [
      pkgs.stdenv.cc.cc.lib
      pkgs.llvmPackages.libclang.lib
    ];  
  }
# TODO: try https://github.com/NixOS/nixpkgs/blob/bc08bb87fa533af4e237f44fd86a8c9af65f55d5/pkgs/development/libraries/rocksdb/default.nix
#   ror: builder for '/nix/store/rah9hl49pgfhd0k8crxzmn52bj0slg3q-composable-picasso-e8640019aa30bd4f689c00bb46e27ed1102fd7de.drv' failed with exit code 1;
#        last 10 log lines:
#        >  'paths': [PosixPath('/nix/store/c183bqnd0g018vrb2f06an6qx8ljqxbx-composable-picasso-e8640019aa30bd4f689c00bb46e27ed1102fd7de')],
#        >  'recursive': True,
#        >  'runtime_dependencies': []***
#        > setting interpreter of /nix/store/c183bqnd0g018vrb2f06an6qx8ljqxbx-composable-picasso-e8640019aa30bd4f689c00bb46e27ed1102fd7de/bin/composable
#        > searching for dependencies of /nix/store/c183bqnd0g018vrb2f06an6qx8ljqxbx-composable-picasso-e8640019aa30bd4f689c00bb46e27ed1102fd7de/bin/composable
#        >     librocksdb.so.7 -> not found!
#        > auto-patchelf: 1 dependencies could not be satisfied
#        > error: auto-patchelf could not satisfy dependency librocksdb.so.7 wanted by /nix/store/c183bqnd0g018vrb2f06an6qx8ljqxbx-composable-picasso-e8640019aa30bd4f689c00bb46e27ed1102fd7de/bin/composable
#        > auto-patchelf failed to find all the required dependencies.
#        > Add the missing dependencies to --libs or use `--ignore-missing="foo.so.1 bar.so etc.so"`.
#        For full logs, run 'nix log /nix/store/rah9hl49pgfhd0k8crxzmn52bj0slg3q-composable-picasso-e8640019aa30bd4f689c00bb46e27ed1102fd7de.drv'.
# error: 1 dependencies of derivation '/nix/store/jskzc0nq6a4k4gzm4b3s0zc5z1113v3n-devnet-polkalaunch.json.drv' failed to build