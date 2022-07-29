import {ApiPromise, WsProvider, Keyring} from '@polkadot/api';
import yargs from "yargs";
import * as fs from "fs";

async function main() {
    const argv = yargs(process.argv.slice(2))
        .usage("Usage: npm run update-release-body [args]")
        .version("1.0.0")
        .options({
            root_key: {
                type: "string",
                describe: "root key to be used to sign txs",
            },
            rpc_ws_url: {
                type: "string",
                describe: "ws/wss url for the node",
            },
            path: {
                type: "string",
                describe: "path to the wasm to upgrade",
            },
        })
        .demandOption(["root_key", "rpc_ws_url", "path"])
        .help().argv
    const keyring = new Keyring({type: 'sr25519'});
    const wsProvider = new WsProvider(argv.rpc_ws_url);
    const api = await ApiPromise.create({provider: wsProvider});
    const root = keyring.addFromUri(argv.root_key);
    let bytes = fs.readFileSync(argv.path).toString('hex')
    let call = api.tx.system.setCode(`0x${bytes}`)

    const unsub = await api.tx.sudo.sudoUncheckedWeight(call, 1)
        .signAndSend(root, (result) => {
                console.log(`Current status is ${result.status}`);
                if (result.status.isInBlock) {
                    console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
                } else if (result.status.isFinalized) {
                    console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
                    unsub();
                    process.exit(0)
                }
            }
        )

    let parachainEvents = 0
    const unsubscribe = await api.query.system.events(events => {
        events.forEach((event) => {
            // parachain_system is index 10 (0x0a00 in hex) in all runtimes
            // we're watching to see if we get the parachainSystem.ValidationFunctionStored
            // and parachainSystem.ValidationFunctionApplied events from parachain_system
            if (event.event.index.toString() === "0x0a00") {
                parachainEvents += 1
                console.log(JSON.stringify(event, undefined, 4))
            }
        })

        if (parachainEvents === 2) {
            unsubscribe()
        }
    })
}

main()