/// example usage of CVM and MANTIS contracts
import { CwXcCoreClient } from "./dist/cw-xc-core/CwXcCore.client.js"
import { GasPrice } from "@cosmjs/stargate"
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate"

import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { Coin } from "cosmjs-types/cosmos/base/v1beta1/coin.js";

const print = console.info

/// NOTE: please note that for details of network prefix and RPC please contact Centauri mainnet support
/// NOTE: this is minimal example with no error handling, syntax sugar or elaborated clients,
/// NOTE: only raw clients and types are used, please contact FE team for React integration
print("creating wallet")
const wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    // replace with your key
    "apart ahead month tennis merge canvas possible cannon lady reward traffic city hamster monitor lesson nasty midnight sniff enough spatial rare multiply keep task",
    {
        // ensure this prefix is actual
        prefix: "centauri",
    }
);

/// at least one account must be created with address to use as sender
const sender = (await wallet.getAccounts())[0].address
print(sender)

print("creating RPC client")
// replace with RPC use really use, this RPC may not work at point you call it"
const rawClient = await SigningCosmWasmClient.connectWithSigner("https://rpc.composable.nodestake.top:443", wallet,
    {
        gasPrice: GasPrice.fromString("0.25ppica")
    })

print("checking CVM contract deployed")

/// update sender according you wallet and contract address in official public CVM docs
const client = new CwXcCoreClient(rawClient, sender, "centauri1c676xpc64x9lxjfsvpn7ajw2agutthe75553ws45k3ld46vy8pts0w203g")
/// check official docs about PICA asset id 
const PICA = await client.getAssetById({ assetId: "158456325028528675187087900673" });
print(PICA)

print("let transfer PICA Centaur to Osmosis")
const msg = {
    executeProgram: {
        assets: [
            ["158456325028528675187087900673", "123456789000"]
        ],
        salt: "737061776e5f776974685f6173736573",
        program: {
            tag: "737061776e5f776974685f6173736574",
            instructions: [
                {
                    spawn: {
                        network_id: 3,
                        salt: "737061776e5f776974685f6173736574",
                        assets: [
                            [
                                "158456325028528675187087900673",
                                {
                                    amount: {
                                        intercept: "123456789000",
                                        slope: "0"
                                    },
                                    is_unit: false
                                }
                            ]
                        ],
                        program: {
                            tag: "737061776e5f776974685f6173736574",
                            instructions: [
                                // so we just transferred PICA to Osmosis virtual wallet
                                // you can encode here transfer to account on Osmosis (from virtual wallet)
                                // or do exchange, see swap-pica-to-osmosis.json
                                // or do raw calls of contracts as per specification
                                // just fill in instructions according shape
                            ]
                        }
                    }
                }
            ]
        }

    },
    tip: "centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k",
}

const coin = Coin.fromPartial({ denom: "ppica", amount: "123456789000" })
const result = await client.executeProgram(msg, "auto", null, [coin])

print(result)