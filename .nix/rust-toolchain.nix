{
  toolchain = rec {
    channel-name = "nightly";
    channel-date = "2022-04-18";
    channel = "${channel-name}-${channel-date}";
    targets = [ "wasm32-unknown-unknown" ];
    extensions = [ "rust-src" "clippy" "rustfmt" ];
  };
}