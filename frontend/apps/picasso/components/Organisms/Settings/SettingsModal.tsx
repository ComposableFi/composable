import { Modal, Select } from "@/components";
import { Box, Button, Card, Stack, Typography } from "@mui/material";
import { useStore } from "@/stores/root";
import React, { useMemo } from "react";
import { AssetId } from "@/defi/polkadot/types";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { setPaymentAsset } from "@/defi/polkadot/pallets/AssetTxPayment";
import { callbackGate } from "shared";
import { useExecutor } from "substrate-react";
import { getAssetOnChainId } from "@/defi/polkadot/Assets";
import { SnackbarKey, useSnackbar } from "notistack";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";

export const SettingsModal = ({
  state,
  onClose
}: {
  state: boolean;
  onClose: () => void;
}) => {
  const feeItem = useStore(state => state.transfers.feeItem);
  const setFeeItem = useStore(state => state.transfers.setFeeItem);
  const fromAssets = useStore(
    ({ substrateBalances }) => substrateBalances.assets.picasso.assets
  );
  const options = useMemo(() => {
    const items = Object.entries(fromAssets).map(([symbol, asset]) => ({
      value: symbol,
      label: asset.meta.name,
      icon: asset.meta.icon,
      disabled: asset.balance.isZero(),
      selected: feeItem === asset.meta.assetId
    }));

    return [
      { value: "", label: "Please select an item", disabled: true },
      ...items
    ];
  }, [feeItem, fromAssets]);

  const handleChangeItem = (item: React.ChangeEvent<HTMLInputElement>) => {
    setFeeItem(item.target.value as AssetId);
  };

  const picassoProvider = usePicassoProvider();
  const account = useSelectedAccount();
  const executor = useExecutor();
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();

  const applyTokenChange = () => {
    const onChainId = getAssetOnChainId("picasso", feeItem);
    callbackGate(
      (api, walletAddress, exec, assetId) => {
        let snackbarId: SnackbarKey | undefined;
        setPaymentAsset({
          api,
          walletAddress,
          assetId,
          executor: exec,
          onSuccess: txHash => {
            closeSnackbar(snackbarId);
            enqueueSnackbar(`Changes successfully stored.`, {
              variant: "success",
              isClosable: true,
              persist: true,
              url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash
            });
          },
          onError: _err => {
            closeSnackbar(snackbarId);
            enqueueSnackbar(`An error occurred while saving settings.`, {
              variant: "error",
              isClosable: true,
              persist: true
            });
          },
          onReady: txHash => {
            snackbarId = enqueueSnackbar(`Saving changes...`, {
              variant: "info",
              isClosable: true,
              persist: true,
              url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash
            });
          }
        });
      },
      picassoProvider.parachainApi,
      account?.address,
      executor,
      onChainId
    );
  };

  return (
    <Modal maxWidth="md" open={state} onClose={onClose} dismissible>
      <Card>
        <Stack direction="column" gap={4}>
          <Typography variant="h6">Settings</Typography>
          <TextWithTooltip tooltip="Token that will be used as gas fee">
            Transfer token
          </TextWithTooltip>
          <Select
            value={feeItem}
            options={options}
            onChange={handleChangeItem}
            disabled={options.length === 1}
          />
          <Box
            display="flex"
            alignItems="center"
            justifyContent="flex-end"
            gap={2}
          >
            <Button variant="outlined" onClick={onClose}>
              Cancel
            </Button>
            <Button
              variant="contained"
              color="primary"
              onClick={applyTokenChange}
            >
              Apply
            </Button>
          </Box>
        </Stack>
      </Card>
    </Modal>
  );
};
