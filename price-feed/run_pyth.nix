with import <nixpkgs> {};
let
  pythd = stdenv.mkDerivation rec {
    name = "pyth-daemon-${version}";
    version = "2.1";
    buildInputs = [ cmake zlib.dev libudev openssl.dev zstd.dev ];
    src = pkgs.fetchFromGitHub {
      repo = "pyth-client";
      owner = "hussein-aitlahcen";
      rev = "update-jsonrpc";
      sha256 = "sha256:1ca8z33pnn6x9dkxii70s1lcskh56fzng1x9lqxzk84q5fffysdb";
    };
    configurePhase = ''
      mkdir build
      cd build
      cmake ..
    '';
    buildPhase = ''
      make
    '';
    installPhase = ''
      mkdir -p $out/bin
      mv pythd $out/bin
      mv pyth_tx $out/bin
    '';
  };
in mkShell {
  packages = [ pythd ];
  SOLANA_ENV = "devnet";
  shellHook = ''
    echo "Running up pyth_tx & pythd"
    export PYTH_TX_LOG=$(mktemp)
    pyth_tx -l $PYTH_TX_LOG -d -r api.$SOLANA_ENV.solana.com &
    export PYTH_TX_PID=$!
    export PYTHD_LOG=$(mktemp)
    pythd -l $PYTHD_LOG -d -r api.$SOLANA_ENV.solana.com &
    export PYTHD_PID=$!
    teardown() {
      echo "Shuting down pyth_tx & pythd";
      kill -2 $PYTHD_PID
      kill -2 $PYTH_TX_PID
      rm $PYTH_TX_LOG
      rm $PYTHD_LOG
    }
    trap teardown EXIT
  '';
}
