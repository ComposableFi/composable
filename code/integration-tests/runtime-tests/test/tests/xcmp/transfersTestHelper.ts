import { ApiPromise, WsProvider } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { Pica } from "@composable/utils/mintingHelper";
import BN from "bn.js";
import { Option } from "@polkadot/types-codec";
import { PalletAssetsAssetAccount } from "@composable/types/interfaces";

export async function initializeApis(newClient: ApiPromise) {
  const picassoApi = newClient;
  const kusamaApi = await initializeKusamaApi();
  const statemineApi = await initializeStatemineApi();
  return { picassoApi, kusamaApi, statemineApi };
}

export async function setChainsForTests(
  relayChainApi: ApiPromise,
  siblingChainApi: ApiPromise,
  chainApi: ApiPromise,
  sudoKey: KeyringPair,
  assetId: number,
  amount: number,
  parachain: number
) {
  await registerAssetOnStatemine(siblingChainApi, sudoKey, assetId);
  const payload = setAssetStatusMessage(siblingChainApi, sudoKey, 5000, assetId);
  await sendXCMMessageFromRelayer(relayChainApi, sudoKey, payload);
  await mintTokensOnStatemine(siblingChainApi, assetId, sudoKey, amount);
  await setMinXcmFee(chainApi, sudoKey, parachain);
}

export async function initializeKusamaApi() {
  const relayChainEndpoint = "ws://" + (process.env.ENDPOINT_RELAYCHAIN ?? "127.0.0.1:9944");
  const relayChainProvider = new WsProvider(relayChainEndpoint);
  const relayChainApiClient = await ApiPromise.create({
    provider: relayChainProvider
  });
  await relayChainApiClient.isReady;
  return relayChainApiClient;
}

async function initializeStatemineApi() {
  const chainEndpoint = "ws://" + (process.env.ENDPOINT_RELAYCHAIN ?? "127.0.0.1:10008");
  const chainProvider = new WsProvider(chainEndpoint);
  const chainApiClient = await ApiPromise.create({
    provider: chainProvider
  });
  await chainApiClient.isReady;
  return chainApiClient;
}

export async function registerAssetOnStatemine(api: ApiPromise, wallet: KeyringPair, assetId: number) {
  const asset = api.createType("Compact<u32>", assetId);
  const admin = api.createType("MultiAddress", {
    Id: api.createType("AccountId", wallet.address)
  });
  const minBalance = api.createType("u128", "10000000");
  await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.assets.Created.is,
    api.tx.assets.create(asset, admin, minBalance)
  );
}

export async function sendXCMMessageFromRelayer(api: ApiPromise, sudoKey: KeyringPair, payload: string) {
  const dest = api.createType("XcmVersionedMultiLocation", {
    V1: api.createType("XcmV1MultiLocation", {
      parents: api.createType("u8", 0),
      interior: api.createType("XcmV1MultilocationJunctions", {
        X1: api.createType("XcmV1Junction", {
          Parachain: api.createType("Compact<u32>", 1000)
        })
      })
    })
  });
  const message = api.createType("XcmVersionedXcm", {
    V1: api.createType("XcmV1Xcm", {
      Transact: {
        originType: api.createType("XcmV0OriginKind", "Superuser"),
        requireWeightAtMost: api.createType("u64", 1000000000),
        call: api.createType("XcmDoubleEncoded", {
          encoded: api.createType("Bytes", payload)
        })
      }
    })
  });
  await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.xcmPallet.Sent.is,
    api.tx.sudo.sudo(api.tx.xcmPallet.send(dest, message))
  );
}

export function to6Digit(amount: string | number) {
  return +amount * 10 ** 6;
}

export async function mintTokensOnStatemine(
  api: ApiPromise,
  assetId: number,
  minterWallet: KeyringPair,
  amountToMint = 1_000,
  receiverWallet = minterWallet
) {
  const id = api.createType("Compact<u32>", assetId);
  const beneficiary = api.createType("MultiAddress", {
    Id: api.createType("AccountId", receiverWallet.address)
  });
  const amount = api.createType("Compact<u128>", to6Digit(amountToMint));
  await sendAndWaitForSuccess(
    api,
    minterWallet,
    api.events.assets.Issued.is,
    api.tx.assets.mint(id, beneficiary, amount)
  );
}

async function setMinXcmFee(api: ApiPromise, sudoKey: KeyringPair, parachain: number) {
  const parachainId = api.createType("u32", parachain);
  const foreignAssetId = api.createType("ComposableTraitsXcmAssetsXcmAssetLocation", {
    parents: api.createType("u8", 1),
    interior: api.createType("XcmV1MultilocationJunctions", {
      X1: api.createType("XcmV1Junction", {
        Parachain: api.createType("Compact<u32>", parachain)
      })
    })
  });
  const amount = api.createType("u128", 600_000_000);
  await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.assetsRegistry.MinFeeUpdated.is,
    api.tx.sudo.sudo(api.tx.assetsRegistry.setMinFee(parachainId, foreignAssetId, amount))
  );
}

export async function disconnectApis(...chains: ApiPromise[]) {
  chains.map(async chain => {
    await chain.disconnect();
  });
}

export async function sendFundsFromRelaychain(
  api: ApiPromise,
  senderWallet: KeyringPair,
  amount: number,
  receiverWallet = senderWallet
) {
  const destination = setDestination(api, 0, 2087);
  const beneficiary = setBeneficiary(api, receiverWallet, 0);
  const paraAmount = api.createType("Compact<u128>", Pica(amount));
  const assets = api.createType("XcmVersionedMultiAssets", {
    V1: api.createType("XcmV1MultiassetMultiAssets", [
      api.createType("XcmV1MultiAsset", {
        id: api.createType("XcmV1MultiassetAssetId", {
          Concrete: api.createType("XcmV1MultiLocation", {
            parents: api.createType("u8", 0),
            interior: api.createType("XcmV1MultilocationJunctions", "Here")
          })
        }),
        fun: api.createType("XcmV1MultiassetFungibility", {
          Fungible: paraAmount
        })
      })
    ])
  });
  const feeAssetItem = api.createType("u32", 0);
  return await sendAndWaitForSuccess(
    api,
    senderWallet,
    api.events.xcmPallet.Attempted.is,
    api.tx.xcmPallet.reserveTransferAssets(destination, beneficiary, assets, feeAssetItem)
  );
}

function setDestination(api: ApiPromise, parent: number, targetParachainId?: number) {
  debugger;
  if (!targetParachainId) {
    return api.createType("XcmVersionedMultiLocation", {
      V1: api.createType("XcmV1MultiLocation", {
        parents: api.createType("u8", parent),
        interior: api.createType("XcmV1MultilocationJunctions", "Here")
      })
    });
  }
  return api.createType("XcmVersionedMultiLocation", {
    V1: api.createType("XcmV1MultiLocation", {
      parents: api.createType("u8", parent),
      interior: api.createType("XcmV1MultilocationJunctions", {
        X1: api.createType("XcmV1Junction", {
          Parachain: api.createType("Compact<u32>", targetParachainId)
        })
      })
    })
  });
}

function setBeneficiary(api: ApiPromise, receiverWallet: KeyringPair, parents: number) {
  return api.createType("XcmVersionedMultiLocation", {
    V1: api.createType("XcmV1MultiLocation", {
      parents: api.createType("u8", parents),
      interior: api.createType("XcmV1MultilocationJunctions", {
        X1: api.createType("XcmV1Junction", {
          AccountId32: {
            network: api.createType("XcmV0JunctionNetworkId", "Any"),
            id: receiverWallet.publicKey
          }
        })
      })
    })
  });
}

export async function fetchTokenBalance(api: ApiPromise, wallet: KeyringPair | string, tokenId: number) {
  if (typeof wallet === "string") {
    const { free } = await api.query.tokens.accounts(wallet, tokenId);
    return new BN(free);
  }
  const { free } = await api.query.tokens.accounts(wallet.publicKey, tokenId);
  return new BN(free);
}

export function calculateTransferredAmount(transferredAmount: number, chainXTransferFee: number) {
  return new BN(transferredAmount * 10 ** 12).sub(new BN(chainXTransferFee));
}

export async function changeGasToken(api: ApiPromise, wallet: KeyringPair, assetId: number) {
  const payer = wallet.publicKey;
  const asset = api.createType("Option<u128>", assetId);
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.tokens.Reserved.is,
    api.tx.assetTxPayment.setPaymentAsset(payer, asset)
  );
}

export async function sendPicaToAnotherAccount(
  api: ApiPromise,
  senderWallet: KeyringPair,
  receiverWallet: KeyringPair,
  amount: number
) {
  const asset = api.createType("u128", 1);
  const dest = api.createType("MultiAddress", {
    Id: api.createType("AccountId", receiverWallet.publicKey)
  });
  const amountP = api.createType("u128", Pica(amount));
  const keepAlive = api.createType("bool", "No");
  return await sendAndWaitForSuccess(
    api,
    senderWallet,
    api.events.balances.Transfer.is,
    api.tx.assets.transfer(asset, dest, amountP, keepAlive)
  );
}

export async function fetchTotalIssuance(api: ApiPromise, assetId: number) {
  const balance = await api.query.tokens.totalIssuance(api.createType("u128", assetId));
  return new BN(balance.toString());
}

export async function fetchXChainTokenBalances(
  apis: ApiPromise[],
  wallets: (string | KeyringPair)[],
  assetId: number[]
) {
  const promises = apis
    .map((api, index) => {
      return wallets.map(wallet => {
        if (assetId[index] === 0) {
          return fetchNativeBalance(api, wallet);
        } else if (assetId[index] === 1984 || assetId[index] === 1985) {
          return fetchTokenBalanceOnStatemine(api, wallet, assetId[index]);
        }
        return fetchTokenBalance(api, wallet, assetId[index]);
      });
    })
    .flat();
  return await Promise.all(promises);
}

export async function sendAssetToRelaychain(
  api: ApiPromise,
  senderWallet: KeyringPair,
  receiverWallet: KeyringPair,
  amount: number,
  destinationWeight: number
) {
  const currencyId = api.createType("u128", 4);
  const amountP = api.createType("u128", Pica(amount));
  const dest = setBeneficiary(api, receiverWallet, 1);
  await sendAndWaitForSuccess(
    api,
    senderWallet,
    api.events.xTokens.TransferredMultiAssets.is,
    api.tx.xTokens.transfer(currencyId, amountP, dest, destinationWeight)
  );
}

export async function fetchNativeBalance(api: ApiPromise, wallet: KeyringPair | string) {
  if (typeof wallet === "string") {
    const accountId = api.createType("AccountId32", wallet);
    const balanceRaw = await api.query.system.account(accountId);
    return new BN(balanceRaw.data.free.toString());
  }
  const accountId = api.createType("AccountId32", wallet.publicKey);
  const balanceRaw = await api.query.system.account(accountId);
  return new BN(balanceRaw.data.free.toString());
}

export async function sendKSMFromStatemine(
  api: ApiPromise,
  senderWallet: KeyringPair,
  receiverWallet: KeyringPair,
  amount: number
) {
  const dest = setDestination(api, 1, 2087);
  const beneficiary = setBeneficiary(api, receiverWallet, 0);
  const paraAmount = api.createType("u128", Pica(amount));
  const asset = api.createType("XcmVersionedMultiAssets", {
    V1: api.createType("XcmV1MultiassetMultiAssets", [
      api.createType("XcmV1MultiAsset", {
        id: api.createType("XcmV1MultiassetAssetId", {
          Concrete: api.createType("XcmV1MultiLocation", {
            parents: api.createType("u8", 1),
            interior: api.createType("XcmV1MultilocationJunctions", "Here")
          })
        }),
        fun: api.createType("XcmV1MultiassetFungibility", {
          Fungible: paraAmount
        })
      })
    ])
  });
  const feeAssetItem = api.createType("u32", 0);
  return await sendAndWaitForSuccess(
    api,
    senderWallet,
    api.events.polkadotXcm.Attempted.is,
    api.tx.polkadotXcm.reserveTransferAssets(dest, beneficiary, asset, feeAssetItem)
  );
}

export async function fetchTokenBalanceOnStatemine(api: ApiPromise, wallet: KeyringPair | string, assetId: number) {
  const asset = api.createType("u32", assetId);
  const account =
    typeof wallet === "string"
      ? api.createType("AccountId32", wallet)
      : api.createType("AccountId32", wallet.publicKey);
  const result = (await api.query.assets.account(asset, account)) as Option<PalletAssetsAssetAccount>;
  return new BN(result.unwrap().balance.toString());
}

export async function sendUSDTFromStatemine(
  api: ApiPromise,
  senderWallet: KeyringPair,
  receiverWallet: KeyringPair,
  amount: number
) {
  const dest = setDestination(api, 1, 2087);
  const beneficiary = setBeneficiary(api, receiverWallet, 0);
  const paraAmount = api.createType("u128", to6Digit(amount));
  const asset = api.createType("XcmVersionedMultiAssets", {
    V1: api.createType("XcmV1MultiassetMultiAssets", [
      api.createType("XcmV1MultiAsset", {
        id: api.createType("XcmV1MultiassetAssetId", {
          Concrete: api.createType("XcmV1MultiLocation", {
            parents: api.createType("u8", 0),
            interior: api.createType("XcmV1MultilocationJunctions", {
              X2: [
                api.createType("XcmV1Junction", {
                  PalletInstance: api.createType("u8", 50)
                }),
                api.createType("XcmV1Junction", {
                  GeneralIndex: api.createType("Compact<u128>", 1984)
                })
              ]
            })
          })
        }),
        fun: api.createType("XcmV1MultiassetFungibility", {
          Fungible: paraAmount
        })
      })
    ])
  });
  const feeAssetItem = api.createType("u32", 0);
  const weightLimit = api.createType("XcmV2WeightLimit", "Unlimited");
  return await sendAndWaitForSuccess(
    api,
    senderWallet,
    api.events.polkadotXcm.Attempted.is,
    api.tx.polkadotXcm.limitedReserveTransferAssets(dest, beneficiary, asset, feeAssetItem, weightLimit)
  );
}

function setTypesForTransfersToStatemine(
  api: ApiPromise,
  senderWallet: KeyringPair,
  receiverWallet: KeyringPair,
  amount: number,
  assetId: number
) {
  const paraAmount = api.createType("u128", to6Digit(amount));
  const asset = api.createType("XcmVersionedMultiAsset", {
    V1: api.createType("XcmV1MultiAsset", {
      id: api.createType("XcmV1MultiassetAssetId", {
        Concrete: api.createType("XcmV1MultiLocation", {
          parents: api.createType("u8", 1),
          interior: api.createType("XcmV1MultilocationJunctions", {
            X3: [
              api.createType("XcmV1Junction", {
                Parachain: api.createType("Compact<u32>", 1000)
              }),
              api.createType("XcmV1Junction", {
                PalletInstance: api.createType("u8", 50)
              }),
              api.createType("XcmV1Junction", {
                GeneralIndex: api.createType("Compact<u128>", assetId)
              })
            ]
          })
        })
      }),
      fun: api.createType("XcmV1MultiassetFungibility", {
        Fungible: api.createType("Compact<u128>", paraAmount)
      })
    })
  });
  const fee = api.createType("XcmVersionedMultiAsset", {
    V1: api.createType("XcmV1MultiAsset", {
      id: api.createType("XcmV1MultiassetAssetId", {
        Concrete: api.createType("XcmV1MultiLocation", {
          parents: api.createType("u8", 1),
          interior: api.createType("XcmV1MultilocationJunctions", "Here")
        })
      }),
      fun: api.createType("XcmV1MultiassetFungibility", {
        Fungible: api.createType("Compact<u128>", 1_000_000_000)
      })
    })
  });
  const dest = api.createType("XcmVersionedMultiLocation", {
    V1: api.createType("XcmV1MultiLocation", {
      parents: api.createType("u8", 1),
      interior: api.createType("XcmV1MultilocationJunctions", {
        X2: [
          api.createType("XcmV1Junction", {
            Parachain: api.createType("Compact<u32>", 1000)
          }),
          api.createType("XcmV1Junction", {
            AccountId32: {
              network: api.createType("XcmV0JunctionNetworkId", "Any"),
              id: receiverWallet.publicKey
            }
          })
        ]
      })
    })
  });
  const weightLimit = api.createType("u64", 9_000_000_000);
  return { asset, fee, dest, weightLimit };
}

export async function sendTokenToStatemine(
  api: ApiPromise,
  senderWallet: KeyringPair,
  receiverWallet: KeyringPair,
  amount: number,
  assetId: number
) {
  const { asset, fee, dest, weightLimit } = setTypesForTransfersToStatemine(
    api,
    senderWallet,
    receiverWallet,
    amount,
    assetId
  );
  return await sendAndWaitForSuccess(
    api,
    senderWallet,
    api.events.xTokens.TransferredMultiAssets.is,
    api.tx.xTokens.transferMultiassetWithFee(asset, fee, dest, weightLimit)
  );
}

export async function trapAssetsOnKusama(api: ApiPromise, sudoKey: KeyringPair, amount: number) {
  const dest = setDestination(api, 1);
  const withdrawAsset = api.createType("XcmV2Instruction", {
    WithdrawAsset: api.createType("XcmV1MultiassetMultiAssets", [
      api.createType("XcmV1MultiAsset", {
        id: api.createType("XcmV1MultiassetAssetId", {
          Concrete: api.createType("XcmV1MultiLocation", {
            parents: api.createType("u8", 0),
            interior: api.createType("XcmV1MultilocationJunctions", "Here")
          })
        }),
        fun: api.createType("XcmV1MultiassetFungibility", {
          Fungible: api.createType("Compact<u128>", Pica(amount))
        })
      })
    ])
  });
  const buyExecution = api.createType("XcmV2Instruction", {
    BuyExecution: {
      fees: api.createType("XcmV1MultiAsset", {
        id: api.createType("XcmV1MultiassetAssetId", {
          Concrete: api.createType("XcmV1MultiLocation", {
            parents: api.createType("u8", 0),
            interior: api.createType("XcmV1MultilocationJunctions", "Here")
          })
        }),
        fun: api.createType("XcmV1MultiassetFungibility", {
          Fungible: api.createType("Compact<u128>", Pica(amount))
        })
      }),
      weightLimit: api.createType("XcmV2WeightLimit", "Unlimited")
    }
  });
  const trap = api.createType("XcmV2Instruction", {
    Trap: api.createType("Compact<u64>", 0)
  });
  const message = api.createType("XcmVersionedXcm", {
    V2: api.createType("XcmV2Xcm", [withdrawAsset, buyExecution, trap])
  });

  await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.relayerXcm.send(dest, message))
  );
}

export async function saveTrappedAssets(api: ApiPromise, sudoKey: KeyringPair, amount: BN) {
  const dest = setDestination(api, 1);
  const claimAsset = api.createType("XcmV2Instruction", {
    ClaimAsset: {
      assets: api.createType("XcmV1MultiassetMultiAssets", [
        api.createType("XcmV1MultiAsset", {
          id: api.createType("XcmV1MultiassetAssetId", {
            Concrete: api.createType("XcmV1MultiLocation", {
              parents: api.createType("u8", 0),
              interior: api.createType("XcmV1MultilocationJunctions", "Here")
            })
          }),
          fun: api.createType("XcmV1MultiassetFungibility", {
            Fungible: api.createType("Compact<u128>", amount)
          })
        })
      ]),
      ticket: setDestination(api, 0)
    }
  });
  const buyExecution = api.createType("XcmV2Instruction", {
    BuyExecution: {
      fees: api.createType("XcmV1MultiAsset", {
        id: api.createType("XcmV1MultiassetAssetId", {
          Concrete: api.createType("XcmV1MultiLocation", {
            parents: api.createType("u8", 0),
            interior: api.createType("XcmV1MultilocationJunctions", "Here")
          })
        }),
        fun: api.createType("XcmV1MultiassetFungibility", {
          Fungible: api.createType("Compact<u128>", amount)
        })
      }),
      weightLimit: api.createType("XcmV2WeightLimit", "Unlimited")
    }
  });
  const depositAsset = api.createType("XcmV2Instruction", {
    DepositAsset: {
      assets: api.createType("XcmV1MultiassetMultiAssetFilter", {
        Wild: api.createType("XcmV1MultiassetWildMultiAsset", "All")
      }),
      maxAssets: api.createType("Compact<u32>", 1),
      beneficiary: api.createType("XcmV1MultiLocation", {
        parents: api.createType("u8", 0),
        interior: api.createType("XcmV1MultilocationJunctions", {
          X1: api.createType("XcmV1Junction", {
            Parachain: api.createType("Compact<u32>", 2087)
          })
        })
      })
    }
  });
  const message = api.createType("XcmVersionedXcm", {
    V2: api.createType("XcmV2Xcm", [claimAsset, buyExecution, depositAsset])
  });

  await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.relayerXcm.send(dest, message))
  );
}

export async function sendUnknownFromStatemine(
  api: ApiPromise,
  senderWallet: KeyringPair,
  amount: number,
  unknownAssetId: number,
  knownAssetId: number
) {
  const dest = setDestination(api, 1, 2087);
  const beneficiary = setBeneficiary(api, senderWallet, 0);
  const usdt = setAssetTypeOnStatemine(api, knownAssetId, amount);
  const unknownAsset = setAssetTypeOnStatemine(api, unknownAssetId, amount);
  const assets = api.createType("XcmVersionedMultiAssets", {
    V1: api.createType("XcmV1MultiassetMultiAssets", [usdt, unknownAsset])
  });
  const feeAssetItem = api.createType("u32", 0);
  await sendAndWaitForSuccess(
    api,
    senderWallet,
    api.events.polkadotXcm.Attempted.is,
    api.tx.polkadotXcm.reserveTransferAssets(dest, beneficiary, assets, feeAssetItem)
  );
}

export function setAssetStatusMessage(
  api: ApiPromise,
  ownerWallet: KeyringPair,
  existentialDeposit: number,
  assetId: number
) {
  const id = api.createType("Compact<u32>", assetId);
  const owner = api.createType("MultiAddress", {
    Id: api.createType("AccountId", ownerWallet.address)
  });
  const issuer = api.createType("MultiAddress", {
    Id: api.createType("AccountId", ownerWallet.address)
  });
  const admin = api.createType("MultiAddress", {
    Id: api.createType("AccountId", ownerWallet.address)
  });
  const freezer = api.createType("MultiAddress", {
    Id: api.createType("AccountId", ownerWallet.address)
  });
  const minBalance = api.createType("Compact<u32>", existentialDeposit);
  const isSufficient = api.createType("bool", true);
  const isFrozen = api.createType("bool", false);
  const encodedCall = api.tx.assets
    .forceAssetStatus(id, owner, issuer, admin, freezer, minBalance, isSufficient, isFrozen)
    .toHex();
  return encodedCall.replace("350204", "");
}

function setAssetTypeOnStatemine(api: ApiPromise, assetId: number, amount: number) {
  return api.createType("XcmV1MultiAsset", {
    id: api.createType("XcmV1MultiassetAssetId", {
      Concrete: api.createType("XcmV1MultiLocation", {
        parents: api.createType("u8", 0),
        interior: api.createType("XcmV1MultilocationJunctions", {
          X2: [
            api.createType("XcmV1Junction", {
              PalletInstance: api.createType("u8", 50)
            }),
            api.createType("XcmV1Junction", {
              GeneralIndex: api.createType("Compact<u128>", assetId)
            })
          ]
        })
      })
    }),
    fun: api.createType("XcmV1MultiassetFungibility", {
      Fungible: api.createType("Compact<u128>", to6Digit(amount))
    })
  });
}
