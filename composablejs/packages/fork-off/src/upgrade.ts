import path from "path";
import fs from "fs";
import { getNewConnection, sendAndWaitForSuccess } from "@composable/utils";
import signale from "signale";

const endpoint = "ws://127.0.0.1:9988";

const runtimeVersion = process.env.UPGRADE_VERSION || "1.10003";
const runtimeUpgradeFilename = "runtime_upgrade.wasm"
const runtimeUpgradePath = path.join(__dirname, "../data", runtimeUpgradeFilename);

async function upgrade() {
  const { newClient: api, newKeyring: keyring } = await getNewConnection(endpoint);

  // Get the current sudo key in the system
  const sudoKey = keyring.addFromUri("//Alice");
  const bobKey = keyring.addFromUri("//Bob");

  const runtimeUpgrade = fs.readFileSync(runtimeUpgradePath);

  if (!runtimeUpgrade) {
    signale.error(`Runtime upgrade file not found. Please place it in the data folder and rename it to ${runtimeUpgradeFilename}`);
    process.exit(1);
  }

  const runtimeUpgradeBytes = fs.readFileSync(runtimeUpgradePath).toString("hex");

  signale.await("Adding funds to the sudo account (Alice)... This step will be removed in the future");

  // Note: this is a hack to get around the fact that the sudo account doesn't have any funds
  // TODO: set sudo balance on genesis
  await sendAndWaitForSuccess(
    api,
    bobKey,
    api.events.balances.Transfer.is,
    api.tx.balances.transfer(sudoKey.address, api.createType("u64", BigInt(100_000_000_000_000_000)))
  );

  signale.success("Funds added to sudo account");

  signale.await(`Upgrading runtime to v${runtimeVersion}...`);

  await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudoUncheckedWeight(api.tx.system.setCode(`0x${runtimeUpgradeBytes}`), 0)
  );

  signale.success("The runtime upgrade has been successfully scheduled");
  signale.await("Waiting for the runtime upgrade to be applied...");

  api.query.system.events(events => {
    // Loop through the Vec<EventRecord>
    events.forEach(async ({ event }) => {
      if (event.method === "ValidationFunctionApplied") {
        signale.success(`Runtime successfully upgraded to version ${runtimeVersion}!`);

        await api.disconnect();
        process.exit(0);
      }
    });
  });
}

upgrade().catch(e => {
  signale.error(e);
  process.exit(1);
});
