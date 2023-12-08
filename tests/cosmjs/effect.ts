import { Effect as E } from "effect"
import { connectComet } from "@cosmjs/tendermint-rpc";
import { QueryClient, setupBankExtension } from "@cosmjs/stargate"
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing"


export const balancesOnOsmosis = (rpc: string, mnemonic: string) => E.tryPromise(async () => {
    const client = await connectComet(rpc)
    const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, { prefix: "osmo" })
    const accounts = await wallet.getAccounts()
    if (accounts[0]) {
        const bank = setupBankExtension(new QueryClient(client))
        return bank.bank.allBalances(accounts[0].address)
    }
    throw new Error("no accounts")
});

