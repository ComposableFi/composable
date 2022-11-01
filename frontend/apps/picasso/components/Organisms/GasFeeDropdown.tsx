import { BaseAsset, Select } from "@/components";
import { getAssetOnChainId } from "@/defi/polkadot/Assets";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import {
  getPaymentAsset,
  setPaymentAsset,
} from "@/defi/polkadot/pallets/AssetTxPayment";
import { AssetId } from "@/defi/polkadot/types";
import { subscribeFeeItemEd } from "@/stores/defi/polkadot/transfers/subscribers";
import { useStore } from "@/stores/root";
import { ErrorOutline, LocalGasStation } from "@mui/icons-material";
import {
  alpha,
  Box,
  CircularProgress,
  InputAdornment,
  Tooltip,
  Typography,
  useTheme,
} from "@mui/material";
import BigNumber from "bignumber.js";
import { SnackbarKey, useSnackbar } from "notistack";
import React, { FC, useCallback, useEffect, useMemo, useRef } from "react";
import { callbackGate } from "shared";
import { useExecutor } from "substrate-react";

type Props = {
  toggleModal: () => void;
  setTargetFeeItem: (feeItem: AssetId) => void;
};
export const GasFeeDropdown: FC<Props> = ({
  toggleModal,
  setTargetFeeItem,
}) => {
  const theme = useTheme();
  const mountRef = useRef(false);
  const feeItem = useStore((state) => state.transfers.feeItem);
  const originalFeeItem = useRef(feeItem);
  const setFeeItem = useStore((state) => state.transfers.setFeeItem);
  const feeItemEd = useStore((state) => state.transfers.feeItemEd);
  const getAssetBalance = useStore(
    (state) => state.substrateBalances.getAssetBalance
  );
  const { parachainApi } = usePicassoProvider();
  const picassoAssets = useStore(
    ({ substrateBalances }) => substrateBalances.assets.picasso.assets
  );
  const options = useMemo(() => {
    return Object.entries(picassoAssets).map(([symbol, asset]) => ({
      value: symbol,
      label: asset.meta.symbol,
      icon: asset.meta.icon,
      disabled: getAssetBalance(asset.meta.assetId, "picasso").isZero(),
      selected: feeItem === asset.meta.assetId,
      assetId: asset.meta.assetId,
    }));
  }, [feeItem, getAssetBalance, picassoAssets]);
  const handleChangeItem = (item: React.ChangeEvent<HTMLInputElement>) => {
    const selectedAssetId = item.target.value as AssetId;
    if (selectedAssetId === feeItem) return;

    toggleModal();
    setTargetFeeItem(selectedAssetId);
    applyTokenChange(selectedAssetId);
  };

  const picassoProvider = usePicassoProvider();
  const account = useSelectedAccount();
  const executor = useExecutor();
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();

  const applyTokenChange = useCallback(
    (assetId: AssetId) => {
      const onChainId = getAssetOnChainId("picasso", assetId);
      return callbackGate(
        async (api, walletAddress, exec, onChainAssetId) => {
          let snackbarId: SnackbarKey | undefined;
          try {
            let successMessage = `You changed your gas token from ${feeItem.toUpperCase()} to ${assetId.toUpperCase()}`;
            await setPaymentAsset({
              api,
              walletAddress,
              assetId: onChainAssetId,
              executor: exec,
              onSuccess: (txHash) => {
                closeSnackbar(snackbarId);
                enqueueSnackbar(`Gas token changed successfully`, {
                  description: successMessage,
                  variant: "success",
                  isClosable: true,
                  persist: true,
                  url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
                });
                originalFeeItem.current = assetId;
                setFeeItem(assetId);
                toggleModal();
              },
              onError: (_err) => {
                closeSnackbar(snackbarId);
                enqueueSnackbar(`An error occurred while saving settings.`, {
                  variant: "error",
                  isClosable: true,
                  persist: true,
                });
                toggleModal();
              },
              onReady: (txHash) => {
                console.log("Executing", txHash);
              },
            });
          } catch {
            // revert fee item
            closeSnackbar(snackbarId);
            enqueueSnackbar(`Operation canceled.`, {
              variant: "warning",
              isClosable: true,
              persist: true,
            });
            toggleModal();
          }
        },
        picassoProvider.parachainApi,
        account?.address,
        executor,
        onChainId
      );
    },
    [
      account?.address,
      closeSnackbar,
      enqueueSnackbar,
      executor,
      feeItem,
      picassoProvider.parachainApi,
      setFeeItem,
      toggleModal,
    ]
  );

  useEffect(() => {
    let unsub: Array<() => void>;
    unsub = [];
    if (parachainApi) {
      subscribeFeeItemEd(parachainApi).then((unsubscribe) => {
        unsub.push(unsubscribe);
      });
    }

    return () => {
      unsub.forEach((call) => call());
    };
  }, [parachainApi]);
  useEffect(() => {
    callbackGate(
      async (api, walletAddress) => {
        if (!mountRef.current) {
          const result = await getPaymentAsset({
            api,
            walletAddress,
            network: "picasso",
          });
          if (result) {
            setFeeItem(result.assetId);
            mountRef.current = true;
          }
        }
      },
      parachainApi,
      account?.address
    );
  }, [parachainApi, account, setFeeItem]);

  return (
    <Select
      options={options}
      value={feeItem}
      variant="outlined"
      size="small"
      onChange={handleChangeItem}
      renderValue={(value) => {
        if (!parachainApi) return null;
        if (!mountRef.current) return <CircularProgress size={24} />;
        const option = options.find((option) => option.value == value);
        const optionBalance = option
          ? getAssetBalance(option.assetId, "picasso")
          : new BigNumber(0);

        if (!option || optionBalance.lte(feeItemEd) || optionBalance.eq(0)) {
          let reason: string;
          reason = optionBalance.lte(feeItemEd)
            ? "Your current token balance is less than existential deposit for this token"
            : "Your balance is zero, try adding more funds to your wallet.";
          return (
            <Box
              sx={{
                minWidth: theme.spacing(8),
              }}
              color={theme.palette.error.main}
            >
              <Tooltip
                title={
                  <>
                    <Typography>Wrong gas token for this transfer.</Typography>
                    <Typography variant="caption">{reason}</Typography>
                  </>
                }
                placement="bottom"
              >
                <ErrorOutline />
              </Tooltip>
            </Box>
          );
        }

        return (
          option && (
            <BaseAsset
              label={option.label || option.value}
              icon={option?.icon}
            />
          )
        );
      }}
      InputProps={{
        startAdornment: (
          <InputAdornment
            position="start"
            sx={{
              marginRight: 0,
            }}
          >
            <LocalGasStation
              sx={{
                width: "3rem",
                color: alpha(theme.palette.common.white, 0.6),
              }}
            />
          </InputAdornment>
        ),
      }}
    />
  );
};
