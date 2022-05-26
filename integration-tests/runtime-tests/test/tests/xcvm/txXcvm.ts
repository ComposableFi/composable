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

describe('XCVM', function() {
  it('Works', async function() {
    this.timeout(60 * 10 * 1000);
    const {
      newClient: api,
      newKeyring
    } = await getNewConnection();
    const {
      devWalletAlice,
      devWalletDave,
      devWalletCharlie,
    } = getDevWallets(newKeyring);

    // Setup mosaic
    const sudoKey = devWalletAlice;
    const relayer = devWalletDave;
    const mosaicEthereumNetworkId = api.createType("u128", 0x1337);
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
    await sendAndWaitForSuccess(
      api,
      devWalletCharlie,
      api.events.xcvm.Executed.is,
      api.tx.xcvm.execute(api.createType("Bytes", "0x0ad2010a3e0a3c0a221220d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d1216080112120a1000901092febf040000000000000000000a3e0a3c0a2212208eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a481216080112120a1000806bbd15bf040000000000000000000a50224e08021216080112120a100010a5d4e800000000000000000000001a320a300a16121401010101010101010101010101010101010101011216080112120a100010a5d4e80000000000000000000000"))
    );
    expect(true).to.be.true
  })
});
