{ pkgs,
  fetchFromGitHub,
  fetchurl,
  composable,
  polkadot,
}:
let
  polkalaunch = pkgs.callPackage (pkgs.stdenv.mkDerivation {
    name = "polkadot-launch";
    version = "1.0.0";
    src = fetchFromGitHub {
      owner = "paritytech";
      repo = "polkadot-launch";
      rev = "99c395b9e7dc7468a4b755440d67e317370974c4";
      hash = "sha256:0is74ad9khbqivnnqfarm8012jvbpg5mcs2p9gl9bz1p7sz1f97d";
    };
    patches = [ ./polkadot-launch.patch ];
    installPhase = ''
      mkdir $out
      cp -r * $out
    '';
  }) {};

  polkadot-bin = pkgs.stdenv.mkDerivation {
    name = "polkadot-${polkadot.version}";
    version = polkadot.version;
    src = fetchurl {
      url = "https://github.com/paritytech/polkadot/releases/download/v${polkadot.version}/polkadot";
      sha256 = polkadot.hash;
    };
    nativeBuildInputs = [
      pkgs.autoPatchelfHook
    ];
    buildInputs = [ pkgs.stdenv.cc.cc ];
    dontUnpack = true;
    installPhase = ''
      mkdir -p $out/bin
      cp $src $out/bin/polkadot
      chmod +x $out/bin/polkadot
    '';
  };

  composable-bin = pkgs.stdenv.mkDerivation rec {
    name = "composable-${composable.name}-${composable.version}";
    version = composable.version;
    src = fetchurl {
      url = "https://storage.googleapis.com/composable-binaries/community-releases/${composable.name}/${name}.tar.gz";
      sha256 = composable.hash;
    };
    nativeBuildInputs = [
      pkgs.autoPatchelfHook
    ];
    buildInputs = [ pkgs.stdenv.cc.cc pkgs.zlib ];
    installPhase = ''
      tar -xvf $src
      mkdir -p $out/bin
      mv release/composable $out/bin
      mv doc $out
    '';
  };

  book = pkgs.stdenv.mkDerivation {
    name = "composable-book";
    src = fetchFromGitHub {
      owner = "ComposableFi";
      repo = "composable";
      rev = composable.version;
      sha256 = composable.revhash;
    };
    buildInputs = [ pkgs.mdbook ];
    phases = [ "installPhase" ];
    installPhase = ''
      mkdir -p $out/book
      cd $src/book
      mdbook build --dest-dir $out/book
    '';
  };

  make-node = tmp-directory: node-type: { name, wsPort, port }: {
    inherit name;
    inherit wsPort;
    inherit port;
    basePath = "${tmp-directory}/${node-type}/${name}";
  };

  make-polkalaunch-config =
    { tmp-directory, relaychain-spec, relaychain-bin, parachain-spec, parachain-bin }: {
      relaychain = {
        bin = relaychain-bin;
        chain = relaychain-spec;
        nodes = map (make-node tmp-directory "relaychain") polkadot.nodes;
      };
      parachains = [
        {
          bin = parachain-bin;
          balance = "1000000000000000000000";
          chain = parachain-spec;
          nodes =
            map (node:
              (make-node tmp-directory "parachain" node) // {
                flags = ["--" "--execution=wasm"];
              }) composable.nodes;
        }
      ];
      types = {};
      finalization = false;
      simpleParachains = [];
    };

  tmp-directory = "/tmp/polkadot-launch";

  devnet-polkalaunch-config =
    pkgs.writeTextFile {
      name = "devnet-polkalaunch.json";
      text = builtins.toJSON (
        make-polkalaunch-config
          { inherit tmp-directory;
            relaychain-spec = polkadot.spec;
            relaychain-bin = "${polkadot-bin}/bin/polkadot";
            parachain-spec = composable.spec;
            parachain-bin = "${composable-bin}/bin/composable";
          }
      );
    };
in {
  script =
    pkgs.writeShellScriptBin "run-${composable.spec}" ''
      rm -rf ${tmp-directory}
      ${polkalaunch}/bin/polkadot-launch ${devnet-polkalaunch-config}
    '';
  documentation = "${composable-bin}/share";
  inherit book;
}
