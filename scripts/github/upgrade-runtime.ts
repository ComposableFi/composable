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

    let unsub = await api.tx.sudo.sudoUncheckedWeight(call, 1)
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
}

main()