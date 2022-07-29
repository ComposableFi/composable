import React, { useContext, useMemo } from "react";
import { NextPage } from "next";
import { Button, Grid, Typography } from "@mui/material";
import { useStore } from "@/stores/root";
import Default from "@/components/Templates/Default";
import {
  gridContainerStyle,
  gridItemStyle,
} from "@/components/Organisms/Transfer/transfer-styles";
import { Header } from "@/components/Organisms/Transfer/Header";
import { TransferNetworkSelector } from "@/components/Organisms/Transfer/TransferNetworkSelector";
import { AmountTokenDropdown } from "@/components/Organisms/Transfer/AmountTokenDropdown";
import { TransferRecipientDropdown } from "@/components/Organisms/Transfer/TransferRecipientDropdown";
import { TransferFeeDisplay } from "@/components/Organisms/Transfer/TransferFeeDisplay";
import { TransferKeepAliveSwitch } from "@/components/Organisms/Transfer/TransferKeepAliveSwitch";
import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import { toChainIdUnit } from "@/defi/polkadot/pallets/BondedFinance";
import { getSigner, useExecutor } from "substrate-react";
import { APP_NAME } from "@/defi/polkadot/constants";
import { useSelectedAccount } from "@/defi/polkadot/hooks";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { useSnackbar } from "notistack";
import { Assets } from "@/defi/polkadot/Assets";
import { balance } from "@/stores/defi/stats/dummyData";
import { getTransferToken } from "@/components/Organisms/Transfer/utils";

const Transfers: NextPage = () => {
  const { enqueueSnackbar } = useSnackbar();
  const tokenId = useStore((state) => state.transfers.tokenId);
  const amount = useStore((state) => state.transfers.amount);
  const selectedRecipient = useStore(
    (state) => state.transfers.recipients.selected
  );
  const from = useStore((state) => state.transfers.networks.from);
  const to = useStore((state) => state.transfers.networks.to);

  const assets = useStore(
    ({ substrateBalances }) => substrateBalances[from].assets
  );

  const native = useStore(
    ({ substrateBalances }) => substrateBalances[from].native
  );

  const isNativeToNetwork = useMemo(() => {
    return assets[getTransferToken(from, to)].meta.supportedNetwork[from] === 1;
  }, [from, to, assets]);
  const balance = isNativeToNetwork ? native.balance : assets[tokenId].balance;

  const account = useSelectedAccount();

  const providers = useAllParachainProviders();

  const executor = useExecutor();

  const handleTransferFromKusamaToPicasso = async () => {
    const api = providers[from].parachainApi;

    if (!api || !executor || !account) {
      console.error("No API or Executor or account", {
        api,
        executor,
        account,
      });
      return;
    }
    const TARGET_ACCOUNT_ADDRESS = selectedRecipient.length
      ? selectedRecipient
      : account.address;

    const TARGET_PARACHAIN_ID = SUBSTRATE_NETWORKS[to].parachainId;

    const destination = api.createType("XcmVersionedMultiLocation", {
      V0: api.createType("XcmV0MultiLocation", {
        X1: api.createType("XcmV0Junction", {
          Parachain: api.createType("Compact<u32>", TARGET_PARACHAIN_ID),
        }),
      }),
    });

    // Setting the wallet receiving the funds
    const beneficiary = api.createType("XcmVersionedMultiLocation", {
      V0: api.createType("XcmV0MultiLocation", {
        X1: api.createType("XcmV0Junction", {
          AccountId32: {
            network: api.createType("XcmV0JunctionNetworkId", "Any"),
            id: api.createType("AccountId32", TARGET_ACCOUNT_ADDRESS),
          },
        }),
      }),
    });

    const paraAmount = api.createType(
      "Compact<u128>",
      toChainIdUnit(amount).toString()
    );

    // Setting up the asset & amount
    const assets = api.createType("XcmVersionedMultiAssets", {
      V0: [
        api.createType("XcmV0MultiAsset", {
          ConcreteFungible: {
            id: api.createType("XcmV0MultiLocation", "Null"),
            amount: paraAmount,
          },
        }),
      ],
    });

    // Setting the asset which will be used for fees (0 refers to first in asset list)
    const feeAssetItem = api.createType("u32", 0);
    const signer = await getSigner(APP_NAME, account.address);
    await executor.execute(
      api.tx.xcmPallet.reserveTransferAssets(
        destination,
        beneficiary,
        assets,
        feeAssetItem
      ),
      account.address,
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
  };
  const handleTransferFromPicassoToKusama = async () => {
    const api = providers[from].parachainApi;

    if (!api || !executor || !account) {
      console.error("No API or Executor or account", {
        api,
        executor,
        account,
      });
      return;
    }
    const TARGET_ACCOUNT_ADDRESS = selectedRecipient.length
      ? selectedRecipient
      : account.address;

    // Set amount to transfer
    const amountToTransfer = api.createType(
      "u128",
      toChainIdUnit(amount).toString()
    );
    // Set destination. Should have 2 Junctions, first to parent and then to wallet
    const destination = api.createType("XcmVersionedMultiLocation", {
      V0: api.createType("XcmV0MultiLocation", {
        X2: [
          api.createType("XcmV0Junction", "Parent"),
          api.createType("XcmV0Junction", {
            AccountId32: {
              network: api.createType("XcmV0JunctionNetworkId", "Any"),
              id: api.createType("AccountId32", TARGET_ACCOUNT_ADDRESS),
            },
          }),
        ],
      }),
    });

    // Set dest weight
    const destWeight = api.createType("u64", 900000000000); // > 9000000000
    const ksmAssetID = api.createType("SafeRpcWrapper", 4);
    const signer = await getSigner(APP_NAME, account.address);

    await executor.execute(
      api.tx.xTokens.transfer(
        ksmAssetID,
        amountToTransfer,
        destination,
        destWeight
      ),
      account.address,
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
  };
  const handleTransferFromKaruraToPicasso = async () => {
    const api = providers[from].parachainApi;

    if (!api || !executor || !account) {
      console.error("No API or Executor or account", {
        api,
        executor,
        account,
      });
      return;
    }
    const TARGET_ACCOUNT_ADDRESS = selectedRecipient.length
      ? selectedRecipient
      : account.address;

    // Set amount to transfer
    const amountToTransfer = api.createType(
      "u128",
      toChainIdUnit(amount).toString()
    );

    // Set destination. Should have 2 Junctions, first to parent and then to wallet
    const destination = api.createType("XcmVersionedMultiLocation", {
      V0: api.createType("XcmV0MultiLocation", {
        X3: [
          api.createType("XcmV0Junction", "Parent"),
          api.createType("XcmV0Junction", {
            Parachain: api.createType(
              "Compact<u32>",
              SUBSTRATE_NETWORKS[to].parachainId
            ),
          }),
          api.createType("XcmV0Junction", {
            AccountId32: {
              network: api.createType("XcmV0JunctionNetworkId", "Any"),
              id: api.createType("AccountId32", TARGET_ACCOUNT_ADDRESS),
            },
          }),
        ],
      }),
    });

    const currencyId = api.createType("AcalaPrimitivesCurrencyCurrencyId", {
      Token: api.createType("AcalaPrimitivesCurrencyTokenSymbol", "KUSD"),
    });

    const destWeight = api.createType("u64", 900000000000); // > 9000000000

    const signer = await getSigner(APP_NAME, account.address);

    await executor.execute(
      api.tx.xTokens.transfer(
        currencyId,
        amountToTransfer,
        destination,
        destWeight
      ),
      account.address,
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
  };
  const handleTransferFromPicassoToKarura = async () => {
    const api = providers[from].parachainApi;

    if (!api || !executor || !account) {
      console.error("No API or Executor or account", {
        api,
        executor,
        account,
      });
      return;
    }
    const TARGET_ACCOUNT_ADDRESS = selectedRecipient.length
      ? selectedRecipient
      : account.address;

    // Set amount to transfer
    const amountToTransfer = api.createType(
      "u128",
      toChainIdUnit(amount).toString()
    );

    // Set destination. Should have 2 Junctions, first to parent and then to wallet
    const destination = api.createType("XcmVersionedMultiLocation", {
      V0: api.createType("XcmV0MultiLocation", {
        X3: [
          api.createType("XcmV0Junction", "Parent"),
          api.createType("XcmV0Junction", {
            Parachain: api.createType(
              "Compact<u32>",
              SUBSTRATE_NETWORKS[to].parachainId
            ),
          }),
          api.createType("XcmV0Junction", {
            AccountId32: {
              network: api.createType("XcmV0JunctionNetworkId", "Any"),
              id: api.createType("AccountId32", TARGET_ACCOUNT_ADDRESS),
            },
          }),
        ],
      }),
    });

    const kusdAssetId = api.createType(
      "CurrencyId",
      Assets.kusd.supportedNetwork.picasso
    );

    const destWeight = api.createType("u64", 900000000000); // > 9000000000

    const signer = await getSigner(APP_NAME, account.address);

    await executor.execute(
      api.tx.xTokens.transfer(
        kusdAssetId,
        amountToTransfer,
        destination,
        destWeight
      ),
      account.address,
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
  };

  const handleTransfer = async () => {
    switch (`${from}-${to}`) {
      case "kusama-picasso":
        return handleTransferFromKusamaToPicasso();
      case "picasso-kusama":
        return handleTransferFromPicassoToKusama();
      case "karura-picasso":
        return handleTransferFromKaruraToPicasso();
      case "picasso-karura":
        return handleTransferFromPicassoToKarura();
      default:
        return Promise.resolve();
    }
  };

  return (
    <Default>
      <Grid
        container
        sx={gridContainerStyle}
        maxWidth={1032}
        columns={10}
        direction="column"
        justifyContent="center"
      >
        <Grid item {...gridItemStyle("6rem")}>
          <Header />
        </Grid>
        <Grid item {...gridItemStyle()}>
          <TransferNetworkSelector />
        </Grid>
        <Grid item {...gridItemStyle()}>
          <AmountTokenDropdown />
        </Grid>
        <Grid item {...gridItemStyle("1.5rem")}>
          <TransferRecipientDropdown />
        </Grid>
        <Grid item {...gridItemStyle("1.5rem")}>
          <TransferFeeDisplay />
        </Grid>
        <Grid item {...gridItemStyle()}>
          <TransferKeepAliveSwitch />
        </Grid>
        <Grid item {...gridItemStyle("1.5rem")}>
          <Button
            variant="contained"
            color="primary"
            disabled={amount.lte(0) || amount.gt(balance)}
            fullWidth
            onClick={handleTransfer}
          >
            <Typography variant="button">Transfer</Typography>
          </Button>
        </Grid>
      </Grid>
    </Default>
  );
};

export default Transfers;
