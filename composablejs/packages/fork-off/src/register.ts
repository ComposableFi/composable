import path from "path";
import fs from "fs";
import { getNewConnection, sendAndWaitForSuccess } from "@composable/utils";
import chalk from "chalk";

const endpoint = "ws://127.0.0.1:9944";

const genesisStatePath = path.join(__dirname, "../data", "genesis-state");
const genesisWasmPath = path.join(__dirname, "../data", "genesis-wasm");

async function register() {
  const { newClient: api, newKeyring: keyring } = await getNewConnection(endpoint);

  // Get the current sudo key in the system
  const sudoKey = keyring.addFromUri("//Alice");

  const genesisHead = fs.readFileSync(genesisStatePath, "utf8");
  const validationCode = fs.readFileSync(genesisWasmPath, "utf8");

  const parachainsParasParaGenesisArgs = api.registry.createType("PolkadotRuntimeParachainsParasParaGenesisArgs", {
    genesisHead,
    validationCode,
    parachain: true
  });

  console.log(chalk.green("Registering parachain..."));

  await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.parasSudoWrapper.sudoScheduleParaInitialize(2087, parachainsParasParaGenesisArgs))
  );

  await api.disconnect();
  return;
}

register()
  .then(() => {
    console.log(chalk.green("Parathread onboarding! Parachain should start producing blocks in around 2 minutes."));
    console.log(
      `Visit https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/parachains to see the parachain status.\n`
    );
    process.exit(0);
  })
  .catch(e => {
    console.log(chalk.red(e));
    process.exit(1);
  });
