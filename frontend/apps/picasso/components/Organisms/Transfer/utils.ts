import { ApiPromise } from "@polkadot/api";
import Executor from "substrate-react/dist/extrinsics/Executor";
import { u128 } from "@polkadot/types-codec";
import { AnyComponentMap, EnqueueSnackbar } from "notistack";
import { Assets } from "@/defi/polkadot/Assets";
import { getSigner } from "substrate-react";
import { APP_NAME } from "@/defi/polkadot/constants";
import { toChainIdUnit } from "@/defi/polkadot/pallets/BondedFinance";

export type TransferHandlerArgs = {
  api: ApiPromise;
  targetChain: number | 0;
  targetAccount: string;
  amount: u128;
  executor: Executor;
  enqueueSnackbar: EnqueueSnackbar<AnyComponentMap>;
  signerAddress: string;
  hasFeeItem: boolean;
  feeItemId: number | null;
};
export function availableTargetNetwork(
  network: string,
  selectedNetwork: string
) {
  switch (selectedNetwork) {
    case "kusama":
      return network === "picasso";
    case "picasso":
      return network === "kusama" || network === "karura";
    case "karura":
      return network === "picasso";
  }
}

export function getTransferToken(
  fromNetwork: string,
  toNetwork: string
): "ksm" | "kusd" {
  if (fromNetwork === "kusama") return "ksm";
  if (fromNetwork === "karura") return "kusd";
  if (fromNetwork === "picasso") return getTransferToken(toNetwork, "");

  return "ksm";
}

export async function transferPicassoKarura({
  api,
  targetChain,
  targetAccount,
  amount,
  executor,
  enqueueSnackbar,
  signerAddress,
  hasFeeItem,
  feeItemId,
}: TransferHandlerArgs) {
  // Set destination. Should have 2 Junctions, first to parent and then to wallet
  const destination = api.createType("XcmVersionedMultiLocation", {
    V0: api.createType("XcmV0MultiLocation", {
      X3: [
        api.createType("XcmV0Junction", "Parent"),
        api.createType("XcmV0Junction", {
          Parachain: api.createType("Compact<u32>", targetChain),
        }),
        api.createType("XcmV0Junction", {
          AccountId32: {
            network: api.createType("XcmV0JunctionNetworkId", "Any"),
            id: api.createType("AccountId32", targetAccount),
          },
        }),
      ],
    }),
  });

  // TODO: Refactor this logic to parnet
  const transferFunction = hasFeeItem
    ? api.tx.xTokens.transfer
    : api.tx.xTokens.transferMulticurrencies;

  const kusdAssetId = api.createType(
    "CurrencyId",
    Assets.kusd.supportedNetwork.picasso
  );

  const destWeight = api.createType("u64", 900000000000); // > 9000000000

  const signer = await getSigner(APP_NAME, signerAddress);

  await executor.execute(
    transferFunction(kusdAssetId, amount, destination, destWeight),
    signerAddress,
    api,
    signer,
    (txHash) => {
      enqueueSnackbar("Transfer executed", {
        persist: true,
        description: `Transaction hash: ${txHash}`,
        variant: "info",
        isCloseable: true,
      });
    },
    (txHash) => {
      enqueueSnackbar("Transfer executed successfully.", {
        persist: true,
        variant: "success",
        isCloseable: true,
      });
    },
    (err) => {
      enqueueSnackbar("Transfer failed", {
        persist: true,
        description: `Error: ${err}`,
        variant: "error",
        isCloseable: true,
      });
    }
  );
}

export async function transferKaruraPicasso({
  api,
  targetChain,
  targetAccount,
  amount,
  executor,
  enqueueSnackbar,
  signerAddress,
}: TransferHandlerArgs) {
  // Set destination. Should have 2 Junctions, first to parent and then to wallet
  const destination = api.createType("XcmVersionedMultiLocation", {
    V0: api.createType("XcmV0MultiLocation", {
      X3: [
        api.createType("XcmV0Junction", "Parent"),
        api.createType("XcmV0Junction", {
          Parachain: api.createType("Compact<u32>", targetChain),
        }),
        api.createType("XcmV0Junction", {
          AccountId32: {
            network: api.createType("XcmV0JunctionNetworkId", "Any"),
            id: api.createType("AccountId32", targetAccount),
          },
        }),
      ],
    }),
  });

  const currencyId = api.createType("AcalaPrimitivesCurrencyCurrencyId", {
    Token: api.createType("AcalaPrimitivesCurrencyTokenSymbol", "KUSD"),
  });

  const destWeight = api.createType("u64", 900000000000); // > 9000000000

  const signer = await getSigner(APP_NAME, signerAddress);

  await executor.execute(
    api.tx.xTokens.transfer(currencyId, amount, destination, destWeight),
    signerAddress,
    api,
    signer,
    (txHash) => {
      enqueueSnackbar("Transfer executed", {
        persist: true,
        description: `Transaction hash: ${txHash}`,
        variant: "info",
        isCloseable: true,
      });
    },
    (txHash) => {
      enqueueSnackbar("Transfer executed successfully.", {
        persist: true,
        variant: "success",
        isCloseable: true,
      });
    },
    (err) => {
      enqueueSnackbar("Transfer failed", {
        persist: true,
        description: `Error: ${err}`,
        variant: "error",
        isCloseable: true,
      });
    }
  );
}

export async function transferPicassoKusama({
  api,
  targetAccount,
  amount,
  executor,
  enqueueSnackbar,
  signerAddress,
  hasFeeItem,
  feeItemId,
}: TransferHandlerArgs) {
  // Set destination. Should have 2 Junctions, first to parent and then to wallet
  const destination = api.createType("XcmVersionedMultiLocation", {
    V0: api.createType("XcmV0MultiLocation", {
      X2: [
        api.createType("XcmV0Junction", "Parent"),
        api.createType("XcmV0Junction", {
          AccountId32: {
            network: api.createType("XcmV0JunctionNetworkId", "Any"),
            id: api.createType("AccountId32", targetAccount),
          },
        }),
      ],
    }),
  });

  // Set dest weight
  const destWeight = api.createType("u64", 900000000000); // > 9000000000
  const ksmAssetID = api.createType("SafeRpcWrapper", 4);

  const feeItemAssetID = [
    [api.createType("u128", 4), amount], // KSM
    [api.createType("u128", feeItemId), toChainIdUnit(1)], // Asset to be used as fees, minFee should be calculated.
  ];
  //transferMulticurrencies: AugmentedSubmittable<(currencies, feeItem, dest, destWeight) => SubmittableExtrinsic<ApiType>, [Vec<ITuple<[u128, u128]>>, u32, XcmVersionedMultiLocation, u64]>;
  const signer = await getSigner(APP_NAME, signerAddress);

  // TODO: Refactor this logic to parnet
  const transferFunction = hasFeeItem
    ? api.tx.xTokens.transfer
    : api.tx.xTokens.transferMulticurrencies;

  const args = hasFeeItem
    ? [ksmAssetID, amount, destination, destWeight]
    : [feeItemAssetID, 1, destination, destWeight];

  await executor.execute(
    transferFunction(...args),
    signerAddress,
    api,
    signer,
    (txHash) => {
      enqueueSnackbar("Transfer executed", {
        persist: true,
        description: `Transaction hash: ${txHash}`,
        variant: "info",
        isCloseable: true,
      });
    },
    (txHash) => {
      enqueueSnackbar("Transfer executed successfully.", {
        persist: true,
        variant: "success",
        isCloseable: true,
      });
    },
    (err) => {
      enqueueSnackbar("Transfer failed", {
        persist: true,
        description: `Error: ${err}`,
        variant: "error",
        isCloseable: true,
      });
    }
  );
}

export async function transferKusamaPicasso({
  api,
  targetChain,
  targetAccount,
  amount,
  executor,
  enqueueSnackbar,
  signerAddress,
}: TransferHandlerArgs) {
  const destination = api.createType("XcmVersionedMultiLocation", {
    V0: api.createType("XcmV0MultiLocation", {
      X1: api.createType("XcmV0Junction", {
        Parachain: api.createType("Compact<u32>", targetChain),
      }),
    }),
  });

  // Setting the wallet receiving the funds
  const beneficiary = api.createType("XcmVersionedMultiLocation", {
    V0: api.createType("XcmV0MultiLocation", {
      X1: api.createType("XcmV0Junction", {
        AccountId32: {
          network: api.createType("XcmV0JunctionNetworkId", "Any"),
          id: api.createType("AccountId32", targetAccount),
        },
      }),
    }),
  });

  // Setting up the asset & amount
  const assets = api.createType("XcmVersionedMultiAssets", {
    V0: [
      api.createType("XcmV0MultiAsset", {
        ConcreteFungible: {
          id: api.createType("XcmV0MultiLocation", "Null"),
          amount,
        },
      }),
    ],
  });

  // Setting the asset which will be used for fees (0 refers to first in asset list)
  const feeAssetItem = api.createType("u32", 0);
  const signer = await getSigner(APP_NAME, signerAddress);
  await executor.execute(
    api.tx.xcmPallet.reserveTransferAssets(
      destination,
      beneficiary,
      assets,
      feeAssetItem
    ),
    signerAddress,
    api,
    signer,
    (txHash) => {
      enqueueSnackbar("Executing transfer...", {
        persist: true,
        variant: "info",
        timeout: 0,
      });
    },
    (txHash) => {
      enqueueSnackbar("Transfer executed successfully.", {
        persist: true,
        variant: "success",
        isCloseable: true,
      });
    },
    (err) => {
      enqueueSnackbar("Transfer failed", {
        persist: true,
        description: `Error: ${err}`,
        variant: "error",
        isCloseable: true,
      });
    }
  );
}
