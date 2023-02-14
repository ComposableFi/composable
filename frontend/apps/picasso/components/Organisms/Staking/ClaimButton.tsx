import { SnackbarKey, useSnackbar } from "notistack";
import {
  useExecutor,
  usePendingExtrinsic,
  usePicassoProvider,
  useSelectedAccount,
  useSigner,
} from "substrate-react";
import config from "@/constants/config";
import { claimAllPicaRewards } from "@/defi/polkadot/pallets/StakingRewards/claim";
import { subscanExtrinsicLink } from "shared";
import { Button, CircularProgress } from "@mui/material";
import { getClaimableAmount } from "@/stores/defi/polkadot/stakingRewards/accessor";
import { useStore } from "@/stores/root";

export const ClaimButton = () => {
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();
  // Claim Request
  const { parachainApi } = usePicassoProvider();
  const account = useSelectedAccount(config.defaultNetworkId);
  const executor = useExecutor();
  const signer = useSigner();
  const isClaiming = usePendingExtrinsic(
    "batch",
    "utility",
    account?.address ?? "-"
  );
  const claimable = useStore(getClaimableAmount);
  const onClaimButtonClick = () => {
    let snackBarKey: SnackbarKey | undefined = undefined;
    claimAllPicaRewards(
      parachainApi,
      executor,
      signer,
      account?.address,
      (txHash) => {
        snackBarKey = enqueueSnackbar(`Claiming rewards`, {
          variant: "info",
          isClosable: true,
          persist: true,
          url: subscanExtrinsicLink("picasso", txHash),
        });
      },
      (txHash) => {
        closeSnackbar(snackBarKey);
        snackBarKey = enqueueSnackbar(`Claiming rewards`, {
          description: "Successfully claimed rewards.",
          variant: "info",
          isClosable: true,
          persist: true,
          url: subscanExtrinsicLink("picasso", txHash),
        });
      },
      (e) => {
        closeSnackbar(snackBarKey);
        snackBarKey = enqueueSnackbar(e, {
          variant: "error",
          isClosable: true,
          persist: true,
        });
      }
    );
  };

  return (
    <Button
      variant="contained"
      color="primary"
      fullWidth
      disabled={isClaiming || claimable.eq(0)}
      onClick={onClaimButtonClick}
      sx={{
        display: "flex",
        alignItems: "center",
        gap: 1,
      }}
    >
      {isClaiming ? (
        <CircularProgress variant="indeterminate" size={24} />
      ) : (
        "Claim rewards"
      )}
    </Button>
  );
};
