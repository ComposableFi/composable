import fs from "fs";
import {getWallets, initializeApi } from "./apiClient";
import {ApiPromise} from "@polkadot/api";

async function upgradeRuntime(filePath: string, endpoint?: string){
  let api: ApiPromise;
  if(!endpoint){
    api = await initializeApi('ws://127.0.0.1:8000');
  } else {
    api = await initializeApi(endpoint);
  }
  const wasmFilePath = fs.readdirSync(filePath).filter(file => file.includes('.wasm'));
  if (wasmFilePath.length === 0) {
    await api.disconnect();
    throw new Error('ERROR: Target directory doesnt hold wasm file. Exiting');
  }
  const runtimeFile = fs.readFileSync(`${filePath}/${wasmFilePath[0]}`, 'hex');
  const stringed = '0x' + runtimeFile;
  const {sudoKey} = getWallets('upgrade');
  const weight = api.createType('SpWeightsWeightV2Weight', {
    refTime: 0,
    proofSize: 0
  })
  await api.tx.sudo.sudoUncheckedWeight(api.tx.system.setCode(stringed), weight).signAndSend(sudoKey);
  for (let i = 0; i<5; i++){
    await api.tx.system.remarkWithEvent('0x').signAndSend(sudoKey);
  }
  await api.disconnect();
}

const filePath = process.argv[2];
const endpoint = process.argv[3];
upgradeRuntime(filePath, endpoint).then(() => {
  console.log('Runtime upgraded');
}).catch((e: Error) =>{
  console.log(e.message)
})