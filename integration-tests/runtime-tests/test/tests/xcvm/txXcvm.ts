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
    } = getDevWallets(newKeyring);

    const sudoKey = devWalletAlice;
    const relayer = devWalletAlice;
    const mosaicEthereumNetworkId = api.createType("u128", 2);
    const maxTransferSize = 1000000000000000;
    const decayer = api.createType("PalletMosaicDecayBudgetPenaltyDecayer", {
      Linear: api.createType("PalletMosaicDecayLinearDecay", {
        factor: api.createType("u128", 5)
      })
    });
    const localAssetId = api.createType("u128", 1);
    const remoteAssetId = api.createType("CommonMosaicRemoteAssetId", {
      EthereumTokenAddress: api.createType("[u8; 20]", "0x0000000000000000000000000000000000000001")
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
      devWalletAlice,
      api.events.xcvm.Executed.is,
      api.tx.xcvm.executeJson(
        null,
        api.createType("Bytes", 0xCAFEBABE), {
          "1": "10000000000000"
      },
        JSON.stringify({
          "tag": [],
          "instructions": [{
            "spawn": {
              "salt": [],
              "network": 2,
              "assets": {
                "1": {
                  "fixed": "10000000000000"
                }
              },
              "program": {
                "tag": [],
                "instructions": [{
                    "spawn": {
                      "salt": [],
                      "network": 1,
                      "assets": {
                        "1": {
                          "ratio": 100
                        }
                      },
                    "program": {
                      "tag": [],
                      "instructions": [{
                        "spawn": {
                          "salt": [],
                          "network": 2,
                          "assets": {
                            "1": {
                              "fixed": "10000000000000"
                            }
                          },
                          "program": {
                            "tag": [],
                            "instructions": [{
                              "spawn": {
                                "salt": [],
                                "network": 1,
                                "assets": {
                                  "1": {
                                    "ratio": 100
                                  }
                                },
                                "program": {
                                  "tag": [],
                                  "instructions": [],
                                }
                              }
                            }],
                          }
                        }
                      }],
                    }
                  }
                }],
              }
            }
          }],
        }))
    );
    expect(true).to.be.true
  })
});
