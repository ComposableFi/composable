import { program } from 'commander'
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import fs from 'fs';
import { cryptoWaitReady } from '@polkadot/util-crypto';

program
    .command('upgrade')
        .description('Performs a runtime upgrade')
        .option('-w --wss <wss>', 'wss url to a node')
        .option('-k --keyring <keyring>', 'path to keyring or seed to use')
        .option('-r --runtime <runtime>', 'path to runtime wasm')
        .option('-m --mode <mode>', 'either sudo or democracy', )
        .action(async function (options) { await upgrade(options)} );


async function upgrade(options) {
 
    console.log("Starting runtime upgrade");  
    await cryptoWaitReady()
    console.log("crypto intialized")

    // Initialise the provider to connect to the local node
    const provider = new WsProvider(options.wss);
    console.log(options.wss);  

    // Create the API and wait until ready (optional provider passed through)
    const api = await new ApiPromise({ provider }).isReady

    // Find the actual keypair in the keyring (if this is a changed value, the key
    // needs to be added to the keyring before - this assumes we have defaults, i.e.
    // Alice as the key - and this already exists on the test keyring)
    const keyring = new Keyring({ type: 'sr25519' });

    console.log("creating keyring")
    const adminPair = keyring.addFromUri(options.keyring);
    console.log(`Using ${adminPair.address}`);  

    // Retrieve the runtime to upgrade
    const code = fs.readFileSync(options.runtime).toString('hex');

    const proposal = api.tx.system && api.tx.system.setCode
      ? api.tx.system.setCode(`0x${code}`) // For newer versions of Substrate
      : api.tx.consensus.setCode(`0x${code}`); // For previous versions

    if (options.mode == 'sudo') {
      console.log(`Upgrading from ${adminPair.address} using sudo, ${code.length / 2} bytes`);

      // Perform the actual chain upgrade via the sudo module
      api.tx.sudo
          .sudoUncheckedWeight(proposal, 0)
          .signAndSend(adminPair, ({ events = [], status }) => {
          console.log('Proposal status:', status.type);

          if (status.isInBlock) {
              console.error('You have just upgraded your chain');

              console.log('Included at block hash', status.asInBlock.toHex());
              console.log('Events:');

              console.log(JSON.stringify(events.toHuman(), null, 2));
          } else if (status.isFinalized) {
              console.log('Finalized block hash', status.asFinalized.toHex());
              process.exit(0);
          }
          console.log("finished runtime upgrade")
      });
    } else if (options.mode == "democracy") {
          console.error('democracy is currently still unsupported')
    } else {
      console.error("unknown options")
    }
}

program.parse()

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}