/// example usage of MANTIS batch auction order contract
/// for CosmWasm usage see
/// - https://github.com/cosmos/cosmjs/blob/main/packages/cosmwasm-stargate/src/signingcosmwasmclient.spec.ts
/// - https://github.com/CosmWasm/ts-codegen
import { CwMantisOrderClient } from "./dist/cw-mantis-order/CwMantisOrder.client.js"
import { GasPrice } from "@cosmjs/stargate"
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate"
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing"
import { Coin } from "cosmjs-types/cosmos/base/v1beta1/coin.js"

const print = console.info

print("creating wallet")
const wallet = await DirectSecp256k1HdWallet.fromMnemonic(
    "apart ahead month tennis merge canvas possible cannon lady reward traffic city hamster monitor lesson nasty midnight sniff enough spatial rare multiply keep task",
    {
        prefix: "centauri",
    }
)

const sender = (await wallet.getAccounts())[0].address
print(sender)

const rawClient = await SigningCosmWasmClient.connectWithSigner("https://rpc.composable.nodestake.top:443", wallet,
    {
        gasPrice: GasPrice.fromString("0.25ppica")
    })

const client = new CwMantisOrderClient(rawClient, sender, "centauri1c676xpc64x1lxjfsvpn7ajw2agutthe75553ws45k3ld26vy8pts0w203g")

const give = "1100000000"
const wants = "1000000000"

print("one side of want")
const ppica = Coin.fromPartial({ denom: "ppica", amount: give })
print(await client.order({
    msg: {
        timeout: 100,
        wants: {
            denom: "pdemo",
            amount: wants,
        },
    }
},
    "auto",
    null,
    [ppica]
))


print("other side of want")
const pdemo = Coin.fromPartial({ denom: "pdemo", amount: give })
print(await client.order({
    msg: {
        timeout: 100,
        wants: {
            denom: "ppica",
            amount: wants,
        },
    }
},
    "auto",
    null,
    [pdemo]
))

print("observer that give and want of one is more than want and less than give of other")
const orders = await client.getAllOrders()
print(orders)

if (
    orders[0].given.amount > orders[1].msg.wants.amount &&
    orders[1].given.amount > orders[0].msg.wants.amount) {
    print("solver run in background and finds all such matches by limits and coins and sends solutions to contract as COWs")
} else {
    print("solver will send cross chain swaps")
}

print("...observe events or query order with you order until it solved or timeouts")
