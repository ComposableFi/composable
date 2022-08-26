with import <nixpkgs> { };
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
      mv pyth $out/bin
      mv pythd $out/bin
      mv pyth_tx $out/bin
      mv ../pctest/init_key_store.sh $out/bin
    '';
  };
in mkShell {
  buildInputs = [ pythd ];
  SOLANA_ENV = "devnet";
  shellHook = ''
    export PYTHD_KEYSTORE=$HOME/.pythd

    function init_keystore() {
      echo "Creating key store"
      rm -rf $PYTHD_KEYSTORE || true
      mkdir -m 600 -p $PYTHD_KEYSTORE
      echo "Initializing key store"
      ${pythd}/bin/pyth init_key -k $PYTHD_KEYSTORE
      echo "Populating key store"
      ${pythd}/bin/init_key_store.sh $SOLANA_ENV $PYTHD_KEYSTORE
    }

    function run() {
      echo "Running pyth_tx"
      export PYTH_TX_LOG=$(mktemp)
      ${pythd}/bin/pyth_tx -l $PYTH_TX_LOG -d -r api.$SOLANA_ENV.solana.com &
      export PYTH_TX_PID=$!

      echo "Running pythd"
      export PYTHD_LOG=$(mktemp)
      ${pythd}/bin/pythd -k $PYTHD_KEYSTORE -l $PYTHD_LOG -d -r api.$SOLANA_ENV.solana.com &
      export PYTHD_PID=$!

      function teardown() {
        echo "Shuting down pyth_tx & pythd";
        kill -2 $PYTHD_PID
        kill -2 $PYTH_TX_PID
        rm $PYTH_TX_LOG
        rm $PYTHD_LOG
      }

      trap teardown EXIT
    }
  '';
}
