import {
  sendAndWaitForSuccess,
  sendWithBatchAndWaitForSuccess
} from "@composable/utils/polkadotjs";
import {
  getNewConnection
} from "@composable/utils/connectionHelper";
import {
  getDevWallets
} from "@composable/utils/walletHelper";
import {
  expect
} from "chai";
import {
  stringToU8a
} from "@polkadot/util/string";

describe('XCVM', function() {
  it('Works', async function() {
    this.timeout(60 * 10 * 1000);
    const {
      newClient: api,
      newKeyring
    } = await getNewConnection();
    const {
      devWalletAlice,
    } = getDevWallets(newKeyring);

    // Setup mosaic
    const sudoKey = devWalletAlice;
    const relayer = devWalletAlice;
    const mosaicEthereumNetworkId = api.createType("u128", 7603700);
    const maxTransferSize = 1000000000000000;
    const decayer = api.createType("PalletMosaicDecayBudgetPenaltyDecayer", {
      Linear: api.createType("PalletMosaicDecayLinearDecay", {
        factor: api.createType("u128", 5)
      })
    });
    const localAssetId = api.createType("u128", 1);
    const remoteAssetId = api.createType("CommonMosaicRemoteAssetId", {
      EthereumTokenAddress: api.createType("[u8;20]", "0x")
    });
    const budget = maxTransferSize;
    const networkInfo = api.createType("PalletMosaicNetworkInfo", {
      enabled: api.createType("bool", true),
      minTransferSize: api.createType("u128", 0),
      maxTransferSize: api.createType("u128", maxTransferSize)
    });
    await sendWithBatchAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is, [
      api.tx.sudo.sudo(api.tx.mosaic.setRelayer(relayer.address)),
      api.tx.sudo.sudo(api.tx.mosaic.setBudget(localAssetId, budget, decayer)),
    ],
      false
    );
    await sendAndWaitForSuccess(
      api,
      relayer,
      api.events.mosaic.NetworksUpdated.is,
      api.tx.mosaic.setNetwork(mosaicEthereumNetworkId, networkInfo),
    );
    await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.mosaic.updateAssetMapping(localAssetId, mosaicEthereumNetworkId, remoteAssetId))
    );

    // Setup XCVM
    const xcvmEthereumNetworkId = 2;
    await sendAndWaitForSuccess(api, sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(
        api.tx.xcvm.setSatellite(
          xcvmEthereumNetworkId,
          [mosaicEthereumNetworkId, api.createType("ComposableSupportEthereumAddress", "0x")]
        )
      )
    );

    const concatU8a = (a: Uint8Array, b: Uint8Array): Uint8Array => {
      let r = new Uint8Array(a.length + b.length);
      r.set(a);
      r.set(b, a.length);
      return r;
    };

    // Send program
    const palletTypeId = stringToU8a("modl");
    const xcvmPalletId = api.consts.xcvm.palletId;

    const programNonce = 0;

    const index =
      api.createType(
        "(u32, AccountId)", [
          api.createType("u32", programNonce),
          api.createType("AccountId", devWalletAlice.addressRaw)
        ]
      ).toU8a();

    const xcvmProgramAccount =
      api.createType(
        "AccountId",
        concatU8a(concatU8a(palletTypeId, xcvmPalletId), index).slice(0, 32)
      );
    console.log(xcvmProgramAccount.toHuman());

    const amount = 1_000_000_000_000;
    await sendAndWaitForSuccess(
      api,
      devWalletAlice,
      api.events.balances.Endowed.is,
      api.tx.assets.transfer(
        1,
        xcvmProgramAccount,
        amount,
        false
      )
    );

    await sendAndWaitForSuccess(
      api,
      devWalletAlice,
      api.events.xcvm.Executed.is,
      api.tx.xcvm.execute({
        "instructions": [{
          "spawn": {
            "network": 2,
            "assets": {
              "1": amount
            },
            "program": {
              "instructions": [],
              "nonce": programNonce + 1
            }
          }
        }],
        "nonce": programNonce
      })
    );
    expect(true).to.be.true
  })
});
