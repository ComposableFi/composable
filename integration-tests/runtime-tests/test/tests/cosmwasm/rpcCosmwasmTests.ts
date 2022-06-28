import {
  sendAndWaitForSuccess
} from "@composable/utils/polkadotjs";
import {
  AccountId32
} from "@polkadot/types/interfaces";
import {
  u8aToHex,
  u8aToString
} from "@polkadot/util/u8a";
import {
  base64Decode
} from "@polkadot/util-crypto";
import {
  Bytes
} from "@polkadot/types-codec";
import type {
  KeyringPair
} from "@polkadot/keyring/types";
import {
  getDevWallets
} from "@composable/utils/walletHelper";
import {
  getNewConnection
} from "@composable/utils/connectionHelper";
import {
  ApiPromise
} from "@polkadot/api";
import "@composable/types/interfaces";
import { randomInt } from "crypto";

const encodeMsg = (api: ApiPromise, ty: string, msg: Object) => {
  const msg_str = JSON.stringify(msg);
  console.log("Encoding " + ty + ":");
  console.log(msg);
  return api.createType("Bytes", msg_str);
};

const logEvent = (event: Bytes) => {
  console.log("Contract emitted event:")
  console.log(JSON.parse(u8aToString(event)))
};

const cw20 = async function(api: ApiPromise, devWalletAlice: KeyringPair) {
  const code_hash = "0xc51bc9e80fd75f88b10f5549ff14e440e6f446f971613eb33eea353951cd9f10";
  const funds = api.createType("BTreeMap<u128, u128>", {
    1: 1000000000000
  });
  const salt = api.createType("Bytes", randomInt(0xCAFEBABE));
  const gas = api.createType("u64", 300000000000);

  const query = async (contract_address: AccountId32, msg: Object): Promise<Object> => {
    const r = await api.rpc.cosmwasm.query(
      devWalletAlice.address,
      contract_address,
      api.createType("BTreeMap<CustomRpcCurrencyId, CustomRpcBalance>", {}),
      gas,
      null,
      encodeMsg(api, "QueryMsg", msg)
    );
    return JSON.parse(u8aToString(base64Decode(JSON.parse(r.result.asOk.toHuman().toString()).ok.ok)));
  };

  const getBalance = async (contract_address: AccountId32, account: KeyringPair): Promise<any> => {
    return query(contract_address, {
      balance: {
        address: u8aToHex(account.addressRaw)
      }
    });
  };

  // Instantiate the contract
  console.log("Instantiating contract...");
  let {
    data: [_, contract_address]
  } = await sendAndWaitForSuccess(
    api,
    devWalletAlice,
    api.events.cosmwasm.Instantiated.is,
    api.tx.cosmwasm.instantiate(
      funds,
      gas,
      null,
      code_hash,
      encodeMsg(api, "InstantiateMsg", {
        name: "PICASSO",
        symbol: "PICA",
        decimals: 12,
        initial_balances: [],
        mint: {
          minter: "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
          cap: null
        },
        marketing: null
      }),
      salt,
    )
  );

  console.log("Contract instantiated at: " + contract_address);

  console.log("Balance: " + (await getBalance(contract_address, devWalletAlice)).balance + " PICA");

  console.log("Minting PICA on alice wallet...");

  // Execute a mint call
  let {
    data: [_a, event]
  } = await sendAndWaitForSuccess(
    api,
    devWalletAlice,
    api.events.cosmwasm.ContractEmitted.is,
    api.tx.cosmwasm.call(contract_address, funds, gas, null, encodeMsg(api, "ExecuteMsg", {
      mint: {
        recipient: "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
        amount: "5000000000000000"
      }
    }))
  );

  logEvent(event);

  console.log("Balance: " + (await getBalance(contract_address, devWalletAlice)).balance + " PICA");

  console.log("Query token info:");
  const infos = await query(contract_address, {
    token_info: {}
  });
  console.log(infos);

  console.log("Query minter:");
  const minter = await query(contract_address, {
    minter: {}
  });
  console.log(minter);
}

const xcvm = async (api: ApiPromise, devWalletAlice: KeyringPair, devWalletDave: KeyringPair) => {
  const code_hash = "0x22d1cdfcfd73cfb80406eccd1193a80471d89d8140948de09fb76a6a0b545d52";
  const funds = api.createType("BTreeMap<u128, u128>", {
    1: 6_000_000_000_000
  });
  const salt = api.createType("Bytes", randomInt(0xDEADC0DE));
  const gas = api.createType("u64", 300000000000);

  // Instantiate the contract
  console.log("Instantiating contract...");
  let {
    data: [_, contract_address]
  } = await sendAndWaitForSuccess(
    api,
    devWalletAlice,
    api.events.cosmwasm.Instantiated.is,
    api.tx.cosmwasm.instantiate(
      funds,
      gas,
      null,
      code_hash,
      encodeMsg(api, "InstantiateMsg", {}),
      salt,
    )
  );

  // Execute a call
  let {
    data: [_a, event]
  } = await sendAndWaitForSuccess(
    api,
    devWalletAlice,
    api.events.cosmwasm.ContractEmitted.is,
    api.tx.cosmwasm.call(
      contract_address,
      funds,
      gas,
      null,
      encodeMsg(api, "ExecuteMsg", "et_phone_home"))
  );

  logEvent(event);
};

describe('COSMWASM', function() {
  it('Instantiate & Call & Query', async function() {
    this.timeout(60 * 2 * 1000);
    const {
      newClient: api,
      newKeyring
    } = await getNewConnection();
    const {
      devWalletAlice,
      devWalletDave
    } = getDevWallets(newKeyring);

    // await cw20(api, devWalletAlice);
    await xcvm(api, devWalletAlice, devWalletDave);
  });
});
