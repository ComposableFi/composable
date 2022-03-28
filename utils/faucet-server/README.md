# Faucet Server

This is a simple server that sends $DALI tokens to users who shamelessly beg for them on a specified slack (planned: discord) channel

Run it like so:

```bash
SLACK_SIGNING_KEY=xxxxxxx ROOT_KEY=xxxxxxx RUST_LOG=debug faucet-sever --port=8080 # port to run the server on
```


Beg for tokens by posting on the dali faucet channel

```asm
/drip {your ss58 substrate address}
```