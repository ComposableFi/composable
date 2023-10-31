import { CwXcCoreClient } from "./dist/cw-xc-core/CwXcCore.client.js"
import { AssetId, ExecuteProgramMsg, Balance } from "./dist/cw-xc-core/CwXcCore.types.js"
import { GasPrice } from "@cosmjs/stargate"
import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate"

import { decodeTxRaw, DirectSecp256k1HdWallet, Registry } from "@cosmjs/proto-signing";
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
    gasPrice : GasPrice.fromString("0.025uatom")
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
            ["158456325028528675187087900673", "1000000000000"]
        ],
        salt: "virtual wallet drv salt",
        program: {
            tag: "42656",
            instructions: [
                {
                    transfer: {
                        to: "0x42",
                        assets: [
                            ["158456325028528675187087900673",
                                {
                                    amount: {
                                        intercept: "100000000000",
                                        slope: "0",
                                    },
                                    is_unit: false
                                }]]

                    }
                }
            ]
        }

    },
    tip: "string",
}

const coin = Coin.fromPartial({ denom: "ppica", amount: "1000000000000" })
const result = await client.executeProgram(msg, "auto", null, [coin]);

print(result)

