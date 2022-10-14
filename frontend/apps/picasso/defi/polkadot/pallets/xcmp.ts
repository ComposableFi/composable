import { ApiPromise } from "@polkadot/api";
import { Executor, getSigner } from "substrate-react";
import { u128 } from "@polkadot/types-codec";
import { AnyComponentMap, EnqueueSnackbar } from "notistack";
import { Assets } from "@/defi/polkadot/Assets";
import { APP_NAME } from "@/defi/polkadot/constants";
import { toChainIdUnit } from "shared";
import { CurrencyId } from "defi-interfaces";
import { XcmVersionedMultiLocation } from "@polkadot/types/lookup";
import BigNumber from "bignumber.js";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";

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
  weight: BigNumber;
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

export async function getTransferCallKusamaPicasso(
  api: ApiPromise,
  targetChain: number | 0,
  targetAccount: string,
  amount: u128,
  signerAddress: string
) {
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
  const call = api.tx.xcmPallet.reserveTransferAssets(
    destination,
    beneficiary,
    assets,
    feeAssetItem
  );
  return { signer, call };
}

export async function getTransferCallPicassoKarura(
  api: ApiPromise,
  targetChain: number | 0,
  targetAccount: string,
  hasFeeItem: boolean,
  signerAddress: string,
  amount: u128,
  feeItemId: number | null
) {
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

  // TODO: Refactor this logic to parent
  const transferFunction = !hasFeeItem
    ? api.tx.xTokens.transfer
    : api.tx.xTokens.transferMulticurrencies;

  const kusdAssetId = api.createType(
    "CurrencyId",
    Assets.kusd.supportedNetwork.picasso
  );

  const feeItemAssetID = [
    [kusdAssetId, amount], // KUSD
    [
      api.createType("u128", feeItemId),
      api.createType("u128", toChainIdUnit(1).toString()),
    ], // Asset to be used as fees, minFee should be calculated.
  ];

  const destWeight = api.createType("u64", 9000000000); // > 9000000000

  const signer = await getSigner(APP_NAME, signerAddress);

  const args = !hasFeeItem
    ? [kusdAssetId, amount, destination, destWeight]
    : [feeItemAssetID, api.createType("u32", 1), destination, destWeight];

  // @ts-ignore
  const call = transferFunction(...args);
  return { signer, call };
}

export async function getTransferCallPicassoKusama(
  api: ApiPromise,
  targetAccount: string,
  amount: u128,
  feeItemId: number | null,
  signerAddress: string,
  hasFeeItem: boolean
) {
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
  const destWeight = api.createType("u64", 9000000000);
  const ksmAssetID = api.createType("SafeRpcWrapper", 4);

  const feeItemAssetID = [
    [api.createType("u128", 4), amount], // KSM
    [
      api.createType("u128", feeItemId),
      api.createType("u128", toChainIdUnit(1).toString()),
    ], // Asset to be used as fees, minFee should be calculated.
  ];

  const signer = await getSigner(APP_NAME, signerAddress);

  // TODO: Refactor this logic to parent
  const transferFunction = !hasFeeItem
    ? api.tx.xTokens.transfer
    : api.tx.xTokens.transferMulticurrencies;

  const args = !hasFeeItem
    ? [ksmAssetID, amount, destination, destWeight]
    : [feeItemAssetID, api.createType("u32", 1), destination, destWeight];

  // @ts-ignore
  const call = transferFunction(...args);
  return { signer, call };
}

export async function getTransferCallKaruraPicasso(
  api: ApiPromise,
  targetChain: number | 0,
  targetAccount: string,
  signerAddress: string,
  amount: u128
) {
  // Set destination. Should have 2 Junctions, first to parent and then to wallet
  const destination: XcmVersionedMultiLocation = api.createType(
    "XcmVersionedMultiLocation",
    {
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
    }
  );

  const currencyId: CurrencyId = api.createType(
    "AcalaPrimitivesCurrencyCurrencyId",
    {
      Token: api.createType("AcalaPrimitivesCurrencyTokenSymbol", "KUSD"),
    }
  );

  const destWeight = api.createType("u64", 20000000000000); // > 9000000000

  const signer = await getSigner(APP_NAME, signerAddress);

  const call = api.tx.xTokens.transfer(
    currencyId,
    amount,
    destination,
    destWeight
  );
  return { signer, call };
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
  const { signer, call } = await getTransferCallPicassoKarura(
    api,
    targetChain,
    targetAccount,
    hasFeeItem,
    signerAddress,
    amount,
    feeItemId
  );

  await executor.execute(
    call,
    signerAddress,
    api,
    signer,
    (txHash) => {
      enqueueSnackbar("Transfer executed", {
        persist: true,
        description: `Transaction hash: ${txHash}`,
        variant: "info",
        isCloseable: true,
        url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
      });
    },
    (txHash) => {
      enqueueSnackbar("Transfer executed successfully.", {
        persist: true,
        variant: "success",
        isCloseable: true,
        url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
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
  const { signer, call } = await getTransferCallKaruraPicasso(
    api,
    targetChain,
    targetAccount,
    signerAddress,
    amount
  );

  await executor.execute(
    call,
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
  weight,
}: TransferHandlerArgs) {
  const { signer, call } = await getTransferCallPicassoKusama(
    api,
    targetAccount,
    amount,
    feeItemId,
    signerAddress,
    hasFeeItem
  );

  await executor.execute(
    call,
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
  const { signer, call } = await getTransferCallKusamaPicasso(
    api,
    targetChain,
    targetAccount,
    amount,
    signerAddress
  );

  await executor.execute(
    call,
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
