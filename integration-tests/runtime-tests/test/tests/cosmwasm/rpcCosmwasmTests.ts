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
import type { KeyringPair } from "@polkadot/keyring/types";

describe('COSMWASM Tests', function() {
  it('Instantiate & Call & Query', async function() {
    this.timeout(60 * 2 * 1000);

    const code_hash = "0xd94faf9fab0baadd6daa303d98c2bc8190d7f9d2ee562663705f1a4371702eb6";
    const funds = api.createType("BTreeMap<u128, u128>", {
      1: 1000000000000
    });
    const salt = api.createType("Bytes", "0x21");
    const gas = api.createType("u64", 300000000000);

    const encodeMsg = (ty: string, msg: Object) => {
      const msg_str = JSON.stringify(msg);
      console.log("Encoding " + ty + ":");
      console.log(msg);
      return api.createType("Bytes", msg_str);
    };

    const query = async (contract_address: AccountId32, msg: Object): Promise<Object> => {
      const r = await api.rpc.cosmwasm.query(
        walletAlice.address,
        contract_address,
        api.createType("BTreeMap<CustomRpcCurrencyId, CustomRpcBalance>", {}),
        gas,
        null,
        encodeMsg("QueryMsg", msg)
      );
      return JSON.parse(u8aToString(base64Decode(JSON.parse(r.result.asOk.data.toUtf8()).ok.ok)));
    };

    const getBalance = async (contract_address: AccountId32, account: KeyringPair): Promise<any> => {
      return query(contract_address, {
        balance: {
          address: u8aToHex(account.addressRaw)
        }
      });
    };

    const logEvent = (event: Bytes) => {
      console.log("Contract emitted event:")
      console.log(JSON.parse(u8aToString(event)))
    };

    // Instantiate the contract
    console.log("Instantiating contract...");
    let {
      data: [_, contract_address]
    } = await sendAndWaitForSuccess(
      api,
      walletAlice,
      api.events.cosmwasm.Instantiated.is,
      api.tx.cosmwasm.instantiate(
        funds,
        gas,
        null,
        code_hash,
        encodeMsg("InstantiateMsg", {
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

    console.log("Balance: " + (await getBalance(contract_address, walletAlice)).balance + " PICA");

    console.log("Minting PICA on alice wallet...");

    // Execute a mint call
    let {
      data: [_a, event]
    } = await sendAndWaitForSuccess(
      api,
      walletAlice,
      api.events.cosmwasm.ContractEmitted.is,
      api.tx.cosmwasm.call(contract_address, funds, gas, null, encodeMsg("ExecuteMsg", {
        mint: {
          recipient: "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
          amount: "5000000000000000"
        }
      }))
    );

    logEvent(event);

    console.log("Balance: " + (await getBalance(contract_address, walletAlice)).balance + " PICA");

    console.log("Query token info:");
    const infos = await query(contract_address, { token_info: {} });
    console.log(infos);

    console.log("Query minter:");
    const minter = await query(contract_address, { minter: {} });
    console.log(minter);
  });
});
