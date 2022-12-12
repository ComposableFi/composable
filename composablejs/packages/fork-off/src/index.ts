import cliProgress from "cli-progress";
import chalk from "chalk";
import path from "path";
import fs from "fs";
import signale from "signale";
import { execFileSync, execSync } from "child_process";
import { getNewConnection } from "@composable/utils";
import { xxhashAsHex } from "@polkadot/util-crypto";
import { WsProvider } from "@polkadot/api";
import { getKusamaVersion } from "./utils";

// File names
const fileNames = {
  binary: "binary",
  wasm: "runtime.wasm",
  hex: "runtime.hex",
  originalSpec: "genesis.json",
  forkedSpec: "fork.json",
  storage: "storage.json",
  relay: "rococo-local.json",
  relayTemp: "rococo-local-temp.json", // For editing and registering parachain
  genesisState: "genesis-state",
  genesisWasm: "genesis-wasm",
  polkadot: "polkadot"
};

// Input paths
const binaryPath = path.join(__dirname, "../data", fileNames.binary);
const wasmPath = path.join(__dirname, "../data", fileNames.wasm);

// Output paths
const hexPath = path.join(__dirname, "../data", fileNames.hex);
const originalSpecPath = path.join(__dirname, "../data", fileNames.originalSpec);
const forkedSpecPath = path.join(__dirname, "../data", fileNames.forkedSpec);
const storagePath = path.join(__dirname, "../data", fileNames.storage);
const relayPath = path.join(__dirname, "../data", fileNames.relay);
const relayPathTemp = path.join(__dirname, "../data", fileNames.relayTemp);
const genesisStatePath = path.join(__dirname, "../data", fileNames.genesisState);
const genesisWasmPath = path.join(__dirname, "../data", fileNames.genesisWasm);
const polkadotPath = path.join(__dirname, "../data", fileNames.polkadot);

const originalChain = "picasso";
const forkChain = "picasso-dev";
const parachainId = 2087;
const endpoint = "wss://picasso-rpc.composable.finance";

const provider = new WsProvider(endpoint);

const progressBar = new cliProgress.SingleBar({}, cliProgress.Presets.shades_classic);

const prefixes = ["0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9" /* System.Account */];
const skippedModulesPrefix = [
  "System",
  "Session",
  "Aura",
  "AuraExt",
  "CollatorSelection",
  "Grandpa",
  "GrandpaFinality",
  "FinalityTracker",
  "Authorship",
  "ParachainSystem"
];

async function main() {
  const polkadotVersion = (await getKusamaVersion()) || "0.9.33";
  const polkadotUrl = `https://github.com/paritytech/polkadot/releases/download/v${polkadotVersion}/polkadot`;

  // If Polkadot binary is not present or is not the right version, download it
  try {
    if (fs.existsSync(polkadotPath)) {
      // Set the Polkadot binary to be executable, in case it was not
      execFileSync("chmod", ["+x", polkadotPath]);

      const rawVersion = execSync(`${polkadotPath} --version`).toString();
      const existingPolkadotVersion = rawVersion.split(" ")[1].split("-")[0];
      if (existingPolkadotVersion !== polkadotVersion) {
        signale.await(
          `Polkadot binary version is ${existingPolkadotVersion}, updating to version ${polkadotVersion}...`
        );
        execSync(`rm ${polkadotPath}`);
        execSync(`wget ${polkadotUrl} --quiet -P ./data`);
        signale.success("Polkadot binary downloaded successfully");
      }
    } else {
      signale.await(`Downloading Polkadot binary version ${polkadotVersion}`);
      // Download polkadot binary
      execSync(`wget ${polkadotUrl} --quiet -P ./data`);
      signale.success("Polkadot binary downloaded successfully");
    }
  } catch {
    signale.error(
      `Error running existing Polkadot binary.\nPlease delete ${polkadotPath} and try again, or try running 'wget ${polkadotUrl}', or download it manually from https://github.com/paritytech/polkadot/releases`
    );
    process.exit(1);
  }

  // Set the Polkadot binary to be executable
  execFileSync("chmod", ["+x", polkadotPath]);

  // Check that the binary exists
  if (!fs.existsSync(binaryPath)) {
    signale.error(
      "Binary missing. Please copy the binary of your substrate node to the data folder and rename it to 'binary'"
    );
    process.exit(1);
  }

  // Set the binary to be executable
  execFileSync("chmod", ["+x", binaryPath]);

  signale.success("Picasso binary found");

  // Check that the runtime wasm exists
  if (!fs.existsSync(wasmPath)) {
    signale.error(
      `WASM missing. Please copy the WASM blob of your substrate node to the data folder and rename it to '${fileNames.wasm}'`
    );
    process.exit(1);
  }

  // Dump wasm to hex
  execSync("cat " + wasmPath + " | hexdump -ve '/1 \"%02x\"' > " + hexPath);

  signale.success("Picasso WASM found");

  // Connect to the node
  const { newClient: api } = await getNewConnection(endpoint);

  if (fs.existsSync(storagePath)) {
    signale.warn(
      chalk.yellow(
        `Reusing cached storage. Delete ${storagePath} and rerun the script if you want to fetch the latest storage`
      )
    );
  } else {
    // Download state of original chain
    signale.await("Fetching state of original chain...");

    // TODO: refactor this part
    const at = (await api.rpc.chain.getBlockHash()).toString();
    progressBar.start(256, 0);
    const stream = fs.createWriteStream(storagePath, { flags: "a" });
    stream.write("[");
    await fetchChunks("0x", 1, stream, at);
    stream.write("]");
    stream.end();
    progressBar.stop();
  }

  const metadata = await api.rpc.state.getMetadata();
  // Populate the prefixes array
  const modules = metadata.asLatest.pallets;
  modules.forEach(module => {
    if (module.storage) {
      if (!skippedModulesPrefix.includes(module.name.toString())) {
        prefixes.push(xxhashAsHex(module.name.toString(), 128));
      } else {
        signale.info("Skipping module", chalk.blueBright(`${module.name.toString()}`));
      }
    }
  });

  // Generate spec for original chain
  execSync(binaryPath + ` build-spec --chain ${originalChain} --raw --log 0 > ` + originalSpecPath);

  signale.success("Original chain spec generated successfully", chalk.blueBright(`(${originalSpecPath})`));

  // Generate spec for forked chain
  execSync(binaryPath + ` build-spec --chain ${forkChain} --raw --log 0 > ` + forkedSpecPath);

  signale.success("Forked chain spec generated successfully", chalk.blueBright(`(${forkedSpecPath})`));

  let storage: [string, string][];
  try {
    storage = JSON.parse(fs.readFileSync(storagePath, "utf8"));
  } catch {
    signale.error(`Error parsing storage file. Please delete ${storagePath} and try again`);
    process.exit(1);
  }

  const originalSpec = JSON.parse(fs.readFileSync(originalSpecPath, "utf8"));
  const forkedSpec = JSON.parse(fs.readFileSync(forkedSpecPath, "utf8"));

  // Modify forked chain info
  forkedSpec.name = originalSpec.name + "-fork";
  forkedSpec.id = originalSpec.id + "-fork";
  forkedSpec.protocolId = originalSpec.protocolId;
  forkedSpec.chainType = "Local";
  forkedSpec.bootNodes = [];
  forkedSpec.para_id = parachainId;

  // Grab the items to be moved, then iterate through and insert into storage
  storage
    .filter(i => prefixes.some(prefix => i[0].startsWith(prefix)))
    // Overwrite the keys in the spec that are not skipped
    .forEach(([key, value]) => (forkedSpec.genesis.raw.top[key] = value));

  // Delete System.LastRuntimeUpgrade to ensure that the on_runtime_upgrade event is triggered
  delete forkedSpec.genesis.raw.top["0x26aa394eea5630e07c48ae0c9558cef7f9cce9c888469bb1a0dceaa129672ef8"];

  // TODO: set initial balance to Alice

  // Set the code to the current runtime code
  forkedSpec.genesis.raw.top["0x3a636f6465"] = "0x" + fs.readFileSync(hexPath, "utf8").trim();

  // To prevent the validator set from changing mid-test, set Staking.ForceEra to ForceNone ('0x02')
  forkedSpec.genesis.raw.top["0x5f3e4907f716ac89b6347d15ececedcaf7dad0317324aecae8744b87fc95f2f3"] = "0x02";

  // Set sudo key to //Alice, comment out if you would like to keep the original sudo key
  forkedSpec.genesis.raw.top["0x5c0d1176a568c1f92944340dbfed9e9c530ebca703c85910e7164cb7d1c9e47b"] =
    "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";

  fs.writeFileSync(forkedSpecPath, JSON.stringify(forkedSpec, null, 4));

  signale.success("Forked genesis updated successfully", chalk.blueBright(`(${forkedSpecPath})`));

  execSync(`${binaryPath} export-genesis-state --chain ${forkedSpecPath} > ${genesisStatePath}`);
  signale.success("Genesis state generated successfully", chalk.blueBright(`(${genesisStatePath})`));

  execSync(`${binaryPath} export-genesis-wasm --chain ${forkedSpecPath} > ${genesisWasmPath}`);
  signale.success("Genesis wasm generated successfully", chalk.blueBright(`(${genesisWasmPath})`));

  // Generate a temporary chain spec (without --raw, so that it can be easily modified)
  execSync(`${polkadotPath} build-spec --chain rococo-local --disable-default-bootnode --log 0 > ${relayPathTemp}`);

  // Modify the temporary chain spec to include the parachain
  registerParachain();

  // Generate the final chain spec, with --raw (needed for the parachain nodes to start)
  execSync(
    `${polkadotPath} build-spec --chain ${relayPathTemp} --disable-default-bootnode --raw --log 0 > ${relayPath}`
  );

  // Remove temporary chain spec
  execSync(`rm ${relayPathTemp}`);

  signale.success("Relay chain spec generated successfully", chalk.blueBright(`(${relayPath})`));

  signale.complete("All done! You can now start your relay chain and parachain nodes");
}

/**
 * Add the parachain to the relay chain spec
 */
function registerParachain() {
  const relayChainSpec = fs.readFileSync(relayPathTemp, "utf8").trim();
  if (!relayChainSpec) {
    signale.error("Relay chain spec not found");
    process.exit(1);
  }

  const rococo = JSON.parse(relayChainSpec);

  const paras = rococo.genesis?.runtime?.runtime_genesis_config?.paras?.paras;

  if (!paras) {
    signale.error("'Paras' not found in relay chain spec. Aborting...");
  }

  paras.push([
    parachainId,
    {
      genesis_head: fs.readFileSync(genesisStatePath, "utf8").trim(),
      validation_code: fs.readFileSync(genesisWasmPath, "utf8").trim(),
      parachain: true
    }
  ]);

  fs.writeFileSync(relayPathTemp, JSON.stringify(rococo, null, 2));

  signale.success("Parachain registered successfully");
}

main()
  .then(() => {
    process.exit(0);
  })
  .catch(err => {
    signale.error(err.toString());
    process.exit(1);
  });

let chunksFetched = 0;
let separator = false;

// Note: this function comes from the original fork-off-substrate repo, and should be refactored
async function fetchChunks(prefix: string, levelsRemaining: number, stream: fs.WriteStream, at: string) {
  if (levelsRemaining <= 0) {
    const pairs = await provider.send("state_getPairs", [prefix, at]);
    if (pairs.length > 0) {
      separator ? stream.write(",") : (separator = true);
      stream.write(JSON.stringify(pairs).slice(1, -1));
    }
    progressBar.update(++chunksFetched);
    return;
  }

  // Async fetch the last level
  if (process.env.QUICK_MODE && levelsRemaining == 1) {
    const promises = [];
    for (let i = 0; i < 256; i++) {
      promises.push(fetchChunks(prefix + i.toString(16).padStart(2, "0"), levelsRemaining - 1, stream, at));
    }
    await Promise.all(promises);
  } else {
    for (let i = 0; i < 256; i++) {
      await fetchChunks(prefix + i.toString(16).padStart(2, "0"), levelsRemaining - 1, stream, at);
    }
  }
}
