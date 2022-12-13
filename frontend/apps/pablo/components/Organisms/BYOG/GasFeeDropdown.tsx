import { BaseAsset, Select } from "@/components";
import { ErrorOutline, LocalGasStation } from "@mui/icons-material";
import {
  alpha,
  Box,
  InputAdornment,
  Tooltip,
  Typography,
  useTheme,
} from "@mui/material";
import { SnackbarKey, useSnackbar } from "notistack";
import React, { FC, useCallback, useEffect, useMemo, useRef } from "react";
import { Asset, callbackGate } from "shared";
import {
  useDotSamaContext,
  useExecutor,
  useParachainApi,
  useSelectedAccount,
} from "substrate-react";
import { TokenId } from "tokens";
import useStore from "@/store/useStore";
import { SUBSTRATE_NETWORKS } from "shared/defi/constants";
import { subscribeFeeItemEd } from "@/store/byog/subscribers";
import { getPaymentAsset, setPaymentAsset } from "@/defi/utils/byog";
import { Signer } from "@polkadot/api/types";
import BigNumber from "bignumber.js";

type Props = {
  toggleModal: () => void;
  setTargetFeeItem: (feeItem: TokenId) => void;
};
export const GasFeeDropdown: FC<Props> = ({
  toggleModal,
  setTargetFeeItem,
}) => {
  const theme = useTheme();
  const feeItem = useStore((state) => state.byog.feeItem);
  const originalFeeItem = useRef(feeItem);
  const setFeeItem = useStore((state) => state.byog.setFeeItem);
  const feeItemEd = useStore((state) => state.byog.feeItemEd);
  const { signer } = useDotSamaContext();
  const tokens = useStore(({ substrateTokens }) => substrateTokens.tokens);
  const balances = useStore(
    ({ substrateBalances }) => substrateBalances.tokenBalances
  );
  const options = useMemo(() => {
    return Object.entries<Asset>(tokens)
      .filter(([_, asset]) => {
        try {
          asset.getPicassoAssetId();
          return true;
        } catch {
          return false;
        }
      })
      .filter(([tokenId, _]) => {
        return (
          tokenId === SUBSTRATE_NETWORKS.picasso.tokenId ||
          !balances.picasso[tokenId as TokenId].free.isZero()
        );
      })
      .map(([tokenId, asset]) => ({
        value: tokenId,
        label: asset.getSymbol(),
        icon: asset.getIconUrl(),
        disabled: balances.picasso[tokenId as TokenId].free.isZero(),
        selected: feeItem === tokenId,
        tokenId: tokenId,
      }));
  }, [feeItem, balances, tokens]);
  const handleChangeItem = (item: React.ChangeEvent<HTMLInputElement>) => {
    const selectedAssetId = item.target.value as TokenId;
    if (selectedAssetId === feeItem) return;

    toggleModal();
    setTargetFeeItem(selectedAssetId);
    applyTokenChange(selectedAssetId);
  };
  const picassoProvider = useParachainApi("picasso");
  const account = useSelectedAccount("picasso");
  const executor = useExecutor();
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();

  const applyTokenChange = useCallback(
    (tokenId: TokenId) => {
      const onChainId = tokens[tokenId].getIdOnChain("picasso");
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
              onSuccess: (txHash: string) => {
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
      signer,
    ]
  );

  useEffect(() => {
    let unsub: Array<() => void>;
    unsub = [];
    if (
      picassoProvider.parachainApi &&
      picassoProvider.apiStatus === "connected"
    ) {
      subscribeFeeItemEd(picassoProvider.parachainApi).then((unsubscribe) => {
        unsub.push(unsubscribe);
      });
    }

    return () => {
      unsub.forEach((call) => call());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [picassoProvider]);
  useEffect(() => {
    callbackGate(
      async (api, walletAddress) => {
        const tokenId = await getPaymentAsset({
          api,
          walletAddress,
          network: "picasso",
          tokens,
        });
        if (tokenId) {
          setFeeItem(tokenId);
        }
      },
      picassoProvider.parachainApi,
      account?.address
    );
  }, [picassoProvider.parachainApi, account, setFeeItem, tokens]);

  return (
    <Select
      options={options}
      value={feeItem}
      variant="outlined"
      size="small"
      sx={{
        minWidth: "170px",
      }}
      onChange={handleChangeItem}
      renderValue={(value) => {
        if (!picassoProvider.parachainApi) return null;
        const option = options.find((option) => option.value == value);
        const optionBalance = option
          ? balances.picasso[option.tokenId as TokenId].free
          : new BigNumber(0);
        if (!option || optionBalance.lte(feeItemEd) || optionBalance.eq(0)) {
          let reason: string;
          reason = optionBalance.lte(feeItemEd)
            ? "Your current token balance is less than existential deposit for this token"
            : "Your balance is zero, try adding more funds to your wallet.";
          return (
            <Box
              sx={{
                minWidth: theme.spacing(9),
                padding: theme.spacing(0),
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
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
                <>
                  <ErrorOutline />
                </>
              </Tooltip>
            </Box>
          );
        }

        return option && <BaseAsset label={option.label} icon={option?.icon} />;
      }}
      InputProps={{
        size: "small",
        startAdornment: (
          <InputAdornment position="start">
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
