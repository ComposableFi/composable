import { Effect as E, Console, ConfigProvider, pipe, Effect, Schedule } from "effect"
import { Options, Command } from "@effect/cli"
import { GasPrice } from "@cosmjs/stargate"
import { BunContext as Context, Runtime } from "@effect/platform-bun"
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing"
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";

import { balancesOnOsmosis } from "./cosmjs/effect.js"
import { CwCvmRuntimeClient, CwCvmRuntimeQueryClient } from "cvm-cw-types/dist/cw-cvm-runtime/CwCvmRuntime.client.js";
import { assert } from "effect/Console";
import { Coin } from "cosmjs-types/cosmos/base/v1beta1/coin.js";
import { assertIsObject } from "primitive-predicates";

const centauri_rpc = Options.text("centauri-rpc").pipe(
    Options.withDefault("http://localhost:26657")
)

const osmosis_rpc = Options.text("osmosis-rpc").pipe(
    Options.withDefault("http://localhost:38093")
)

const wait_timeout_seconds = Options.integer("wait-timeout-seconds").pipe(
    Options.withDefault(120)
)

const exec_timeout_seconds = Options.integer("exec-timeout-seconds").pipe(
    Options.withDefault(120)
)

const outpost_contract_address = Options.text("outpost-contract-address")

const mnemonic = Options.text("mnemonic")


const more = (before: Coin[], after: Coin[]) =>
    after.some((updated) => {
        console.log(after)
        const old = before.find((b) => b.denom === updated.denom);
        return old === undefined ? true : parseInt(old.amount) < parseInt(updated.amount)
    })


const checkIncreased = (before: Coin[], osmosis_rpc: string, mnemonic: string) =>
    pipe(
        balancesOnOsmosis(osmosis_rpc, mnemonic),        
        E.tap((after) =>
            E.if(
                more(before, after),
                {
                    onFalse: E.fail(new Error("no coins arrived yet")),
                    onTrue: E.succeed("ok")
                }
            )
        )
    )

const retry = (before: Coin[], osmosis_rpc: string, mnemonic: string, exec_timeout_seconds: number) =>
    pipe(
        E.retry(
            checkIncreased(before, osmosis_rpc, mnemonic),
            Schedule.fixed(12_000)
        ),
        E.timeoutFail(
            {
                onTimeout: () => new Error(`timeout after ${exec_timeout_seconds} seconds`),
                duration: exec_timeout_seconds * 1000,
            })
    )

const testTransferToOsmosis = Command.make("test", { centauri_rpc, outpost_contract_address, osmosis_rpc, mnemonic, wait_timeout_seconds, exec_timeout_seconds },
    ({ mnemonic, centauri_rpc, outpost_contract_address, osmosis_rpc, wait_timeout_seconds, exec_timeout_seconds }) =>
        E.Do.pipe(
            E.bind("wallet", (_) => centauriWallet(mnemonic)),
            E.tap((_) =>
                Effect.retry(
                    balancesOnOsmosis(osmosis_rpc, mnemonic),
                    Schedule.fixed(1000)
                ).pipe(E.timeout(wait_timeout_seconds * 1000))
            ),
            E.bind("osmosisBalancesBefore", () => balancesOnOsmosis(osmosis_rpc, mnemonic)),
            E.tap(Console.log),
            E.bind("osmosisBalancesAfter", (_) => balancesOnOsmosis(osmosis_rpc, mnemonic)),
            E.tap(Console.log),
            E.tap(({ wallet }) => E.tryPromise(
                () => executeComposableCosmosPicaToOsmosis(wallet, centauri_rpc, outpost_contract_address))
            ),
            E.tap(({ osmosisBalancesBefore }) => retry(osmosisBalancesBefore, osmosis_rpc, mnemonic, exec_timeout_seconds)),
        )
)


const centauriWallet = (mnemonic: string) =>
    E.promise(() => {
        return DirectSecp256k1HdWallet.fromMnemonic(
            mnemonic,
            {
                prefix: "centauri",
            }
        )
    })


const command = testTransferToOsmosis

const cli = Command.run(command, {
    name: "test",
    version: "0.0.1",
})

E.suspend(() => cli(process.argv.slice(2))).pipe(
    E.withConfigProvider(ConfigProvider.nested(ConfigProvider.fromEnv(), "TESTS")),
    E.provide(Context.layer),
    Runtime.runMain
)

const executeComposableCosmosPicaToOsmosis = async (wallet: DirectSecp256k1HdWallet, rpc: string, address: string) => {
    assert(address.length > 0, "address must be set")
    console.log("cvm::tests:: so we are doing CVM call")
    const cosmWasmClient = await SigningCosmWasmClient.connectWithSigner(rpc, wallet,
        {
            gasPrice: GasPrice.fromString("0.25ppica")
        })
    const accounts = await wallet.getAccounts()
    const account = accounts[0]
    assertIsObject(account)
    const inputAsset = "158456325028528675187087900673"
    const client = new CwCvmRuntimeQueryClient(cosmWasmClient, address)
    const exists = await client.getAssetById({ assetId: inputAsset });
    assert(exists.asset.local !== null, "asset not found")

    const amount = "123456789000"
    const coin = Coin.fromPartial({ denom: "ppica", amount })
    const msg = {
        assets: [
            ["158456325028528675187087900673", amount]
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
                                    intercept: amount,
                                },
                            ]
                        ],
                        program: {
                            tag: "737061776e5f776974685f6173736574",
                            instructions: [
                                {
                                    transfer : {
                                        assets : [
                                            [
                                                "237684487542793012780631851009",
                                                {
                                                    slope: "1000000000000000000",
                                                },
                                            ]
                                        ],
                                        to: {
                                            account : account.address 
                                        }
                                    }
                                }
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
        },
        tip: "centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k",
    }
  
    await new CwCvmRuntimeClient(cosmWasmClient, account.address, address).executeProgram(msg, "auto", undefined, [coin])
}