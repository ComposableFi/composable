import { TokenId } from "tokens";
import { BaseAsset, Select } from "@/components";

import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import {
  getPaymentAsset,
  setPaymentAsset,
} from "@/defi/polkadot/pallets/AssetTxPayment";

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
import { useDotSamaContext, useExecutor } from "substrate-react";
import { Signer } from "@polkadot/api/types";
type Props = {
  toggleModal: () => void;
  setTargetFeeItem: (feeItem: TokenId) => void;
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
  const { signer } = useDotSamaContext();
  const { parachainApi } = usePicassoProvider();
  const tokens = useStore(
    ({ substrateTokens }) => substrateTokens.tokens
  );
  const balances = useStore(
    ({ substrateBalances }) => substrateBalances.balances
  );

  const options = useMemo(() => {
    return Object.values(tokens).map((token) => ({
      value: token.id,
      label: token.symbol,
      icon: token.icon,
      disabled: balances["picasso"][token.id].balance.isZero(),
      selected: feeItem === token.id,
      tokenId: token.id,
    }));
  }, [feeItem, balances, tokens]);
  const handleChangeItem = (item: React.ChangeEvent<HTMLInputElement>) => {
    const selectedAssetId = item.target.value as TokenId;
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
    (tokenId: TokenId) => {
      const onChainId = tokens[tokenId].picassoId;
      return callbackGate(
        async (api, walletAddress, exec, onChainAssetId) => {
          let snackbarId: SnackbarKey | undefined;
          try {
            let successMessage = `You changed your gas token from ${feeItem.toUpperCase()} to ${tokenId.toUpperCase()}`;
            await setPaymentAsset({
              api,
              signer: signer as Signer,
              walletAddress,
              assetId: onChainAssetId.toString(),
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
                originalFeeItem.current = tokenId;
                setFeeItem(tokenId);
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
        onChainId,
        signer
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
      tokens,
      signer
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
            tokens
          });
          if (result) {
            setFeeItem(result.id);
            mountRef.current = true;
          }
        }
      },
      parachainApi,
      account?.address
    );
  }, [parachainApi, account, setFeeItem, tokens]);

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
          ? balances.picasso[option.tokenId].balance
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
