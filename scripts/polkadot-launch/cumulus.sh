# Common good parachains - Statemine, Statemint
(
    cd ../../../
    git clone -b polkadot-v0.9.18 https://github.com/paritytech/cumulus.git
    cargo build --release
)
../../../cumulus/target/release/polkadot-collator --version
