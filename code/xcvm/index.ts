
import { Addr  } from "./dist/cw-mantis-order/CwMantisOrder.types"
import { CwXcCoreClient  } from "./dist/cw-xc-core/CwXcCore.client"
import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate"

import { decodeTxRaw, DirectSecp256k1HdWallet, Registry } from "@cosmjs/proto-signing";

const print = console.info

// replace with RPC use really use, this RPC may not work at point you call it"
const CENTAURI_MAINNET = "https://centauri-mainnet.concordium.software"

print("creating RPC client")

const rawClient = new SigningCosmWasmClient(CENTAURI_MAINNET, "some key")
const client = new CwXcCoreClient(CENTAURI_MAINNET)

print("This is set of examples which teaches you basics of CW and CVM")

print("Let start from simple CVM program to transfer PICA from one account to another")


