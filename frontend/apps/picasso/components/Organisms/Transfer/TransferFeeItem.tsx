import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Button,
  Typography
} from "@mui/material";
import React, { FC, useEffect, useMemo, useState } from "react";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";
import { useStore } from "@/stores/root";
import { Select } from "@/components";
import { AssetId } from "@/defi/polkadot/types";
import { callbackGate } from "shared";
import {
  getPaymentAsset,
  setPaymentAsset
} from "@/defi/polkadot/pallets/AssetTxPayment";
import { useAllParachainProviders } from "@/defi/polkadot/context/hooks";
import { useSelectedAccount } from "@/defi/polkadot/hooks";
import { AssetMetadata, getAssetOnChainId } from "@/defi/polkadot/Assets";
import { useExecutor } from "substrate-react";
import { SnackbarKey, useSnackbar } from "notistack";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";

const headerStyle = {
  "&.MuiAccordionSummary-root": {
    paddingRight: 0
  },

  "&.Mui-expanded": {
    minHeight: "3rem",

    "& > .MuiAccordionSummary-content.Mui-expanded": {
      margin: 0
    }
  }
};

const expandIconStyle = {
  color: "primary.light"
};

const detailsStyle = {
  "&.MuiAccordionDetails-root": {
    marginTop: "1rem",
    marginBottom: "0.5rem"
  }
};

export const TransferFeeItem: FC = () => {
  const { hasFeeItem, feeItem, setFeeItem, toggleHasFee } = useStore(
    ({ transfers }) => transfers
  );
  const [showApplyButton, setShowApplyButton] = useState(false);
  const balances = useStore(
    ({ substrateBalances }) => substrateBalances.assets
  );
  const from = useStore(({ transfers }) => transfers.networks.from);
  const assetOnChainId = getAssetOnChainId(from, feeItem);
  const allProviders = useAllParachainProviders();
  const provider = allProviders[from];
  const account = useSelectedAccount();
  const executor = useExecutor();
  const { closeSnackbar, enqueueSnackbar } = useSnackbar();
  const options = useMemo(() => {
    const { assets } = balances[from];
    const items = Object.entries(assets)
      .filter(([_, asset]) => !asset.balance.isZero())
      .map(([symbol, asset]) => ({
        value: symbol,
        label: asset.meta.name,
        icon: asset.meta.icon,
        disabled: asset.balance.isZero()
      }));

    return [
      { value: "", label: "Please select an item", disabled: true },
      ...items
    ];
  }, [balances, from]);

  // This effect is used to calculate current transaction asset and populate a button
  useEffect(() => {
    if (!hasFeeItem) return;
    const asset = callbackGate(
      (api, walletAddress) =>
        getPaymentAsset({
          api,
          walletAddress
        }),
      provider.parachainApi,
      account?.address
    );
    asset.then((meta: AssetMetadata) => {
      if (meta.assetId !== feeItem) {
        setShowApplyButton(true);
      } else {
        setShowApplyButton(false);
      }
    });
  }, [provider, account, hasFeeItem, feeItem]);

  const handleChangeItem = (item: React.ChangeEvent<HTMLInputElement>) => {
    setFeeItem(item.target.value as AssetId);
  };

  const handlePaymentAssetChange = () =>
    callbackGate(
      (api, executor, walletAddress, assetOnChainId) => {
        let snackbarKey: SnackbarKey | undefined;
        setPaymentAsset({
          api,
          walletAddress,
          assetId: assetOnChainId,
          executor,
          onSuccess: txHash => {
            enqueueSnackbar(
              "Payment asset successfully changed to " + feeItem.toUpperCase(),
              {
                variant: "success",
                isClosable: true,
                persist: true,
                url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash
              }
            );

            closeSnackbar(snackbarKey);
          },
          onError: err => {
            enqueueSnackbar("There was an error changing asset... ", {
              variant: "error",
              isClosable: true,
              persist: true,
              description: err,
              url: SUBSTRATE_NETWORKS.picasso.subscanUrl
            });
            closeSnackbar(snackbarKey);
          },
          onReady: txHash => {
            snackbarKey = enqueueSnackbar("Changing payment asset... ", {
              variant: "info",
              isClosable: true,
              persist: true,
              url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash
            });
          }
        });
      },
      provider.parachainApi,
      executor,
      account?.address,
      assetOnChainId
    );

  if (options.length === 1) {
    return null;
  }

  return (
    <Accordion expanded={hasFeeItem} onChange={toggleHasFee}>
      <AccordionSummary
        sx={headerStyle}
        expandIcon={<ExpandMoreIcon sx={expandIconStyle} />}
        aria-controls="recipient-content"
        id="recipient-header"
      >
        <Typography variant="body2" color="primary.light">
          Pay fee in different asset
        </Typography>
      </AccordionSummary>
      <AccordionDetails sx={detailsStyle}>
        <Select value={feeItem} options={options} onChange={handleChangeItem} />
        {showApplyButton && (
          <Button
            onClick={handlePaymentAssetChange}
            variant="contained"
            color="primary"
          >
            Apply
          </Button>
        )}
      </AccordionDetails>
    </Accordion>
  );
};
