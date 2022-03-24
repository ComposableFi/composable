import './picasso/augment-api';
import './picasso/augment-types';
import {ApiPromise, WsProvider, Keyring} from '@polkadot/api';

async function main() {
    const keyring = new Keyring({type: 'sr25519'});
    const provider = new WsProvider('wss://picasso-rpc.composable.finance');
    const api = await ApiPromise.create({
        provider,
        signedExtensions: {
            PrevalidateAssociation: {
                extrinsic: {},
                payload: {}
            }
        }
    });
    const root = keyring.addFromUri('0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a');


    // doesn't work, signed extension issues
    let unsub = await api.tx.system.remark("hello world")
        .signAndSend(root, (result) => {
            console.log(`Current status is ${result.status}`);
            if (result.status.isInBlock) {
                console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
            } else if (result.status.isFinalized) {
                console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
                unsub();
            }
        }
    )
}

main()