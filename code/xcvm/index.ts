
import { Addr } from "./dist/cw-mantis-order/CwMantisOrder.types.js"
import { CwXcCoreClient } from "./dist/cw-xc-core/CwXcCore.client.js"
import { AssetId } from "./dist/cw-xc-core/CwXcCore.types.js"
import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate"
import { decodeTxRaw, DirectSecp256k1HdWallet, Registry } from "@cosmjs/proto-signing";

const print = console.info
async function main() {
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

    print("creating RPC client")
    // replace with RPC use really use, this RPC may not work at point you call it"
    const rawClient = await SigningCosmWasmClient.connectWithSigner("https://rpc.composable.nodestake.top:443", wallet)

    print("checking CVM contract deployed")

    /// update sender according you wallet and contract address in official public CVM docs
    const client = new CwXcCoreClient(rawClient, "xcvm", "centauri1c676xpc64x9lxjfsvpn7ajw2agutthe75553ws45k3ld46vy8pts0w203g")
    /// check official docs about PICA asset id 
    const PICA = await client.getAssetById({ assetId: "158456325028528675187087900673" });
    print(PICA)
}

// In a module, once the top-level `await` proposal lands
try {
    const result = await main();
    print(result);
} catch (e) {
    console.error(e);
}