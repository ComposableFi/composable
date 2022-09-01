import { alpha, useTheme } from "@mui/material/styles";
import { Box, Button, InputAdornment, Stack, Typography } from "@mui/material";
import { BigNumberInput, TokenAsset } from "@/components";
import { usePicassoProvider, useSelectedAccount } from "@/defi/polkadot/hooks";
import { useActiveBonds } from "@/defi/polkadot/hooks/useActiveBonds";
import PositionDetailsRow from "@/components/Atom/PositionDetailsRow";
import { claim, getROI } from "@/defi/polkadot/pallets/BondedFinance";
import router from "next/router";
import { ActiveBond } from "@/stores/defi/polkadot/bonds/slice";
import { PairAsset } from "@/components/Atom/PairAsset";
import { useExecutor } from "substrate-react";
import { humanBalance } from "shared";
import { useClaim } from "@/stores/defi/polkadot/bonds/useClaim";
import { findCurrentBond } from "@/stores/defi/polkadot/bonds/utils";
import { SnackbarKey, useSnackbar } from "notistack";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";

export const ClaimForm = () => {
  const theme = useTheme();
  const account = useSelectedAccount();
  const { parachainApi } = usePicassoProvider();
  const { activeBonds } = useActiveBonds();
  const executor = useExecutor();
  const { bond } = router.query;
  const { claimable, vestingTime, vestedTime, pending } = useClaim(
    bond?.toString() ?? ""
  );
  const { enqueueSnackbar, closeSnackbar } = useSnackbar();
  const activeBond = activeBonds.find((b: ActiveBond) =>
    findCurrentBond(b, bond?.toString() ?? "")
  );
  if (activeBond === undefined || !parachainApi) return null;

  const handleClaim = () => {
    let snackbarKey: SnackbarKey | undefined;
    claim(
      {
        parachainApi,
        account,
        executor,
        assetId: activeBond.bond.reward.assetId,
        vestingScheduleId: activeBond.vestingScheduleId.toString(),
      },
      (txHash) => {
        closeSnackbar(snackbarKey);
        enqueueSnackbar("Claim was successful", {
          variant: "success",
          isClosable: true,
          persist: true,
          url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
        });
      },
      (msg) => {
        closeSnackbar(snackbarKey);
        enqueueSnackbar("An error occurred while processing transaction", {
          variant: "error",
          isClosable: true,
          persist: true,
          description: msg,
        });
      },
      (txHash) => {
        snackbarKey = enqueueSnackbar("Processing Claim", {
          variant: "info",
          isClosable: true,
          persist: true,
          url: SUBSTRATE_NETWORKS.picasso.subscanUrl + txHash,
        });
      }
    );
  };

  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        backgroundColor: alpha(theme.palette.common.white, 0.02),
        borderRadius: "0.75rem",
        padding: "3rem",
        width: "50%",
        minWidth: "50%",
      }}
    >
      <Typography
        variant="h5"
        color="text.common.white"
        textAlign="left"
        mb="2rem"
      >
        Claim
      </Typography>
      <BigNumberInput
        value={claimable}
        isValid={() => {}}
        setter={() => {}}
        maxValue={claimable}
        disabled={true}
        LabelProps={{
          mainLabelProps: { label: "Amount" },
        }}
        InputProps={{
          startAdornment: (
            <InputAdornment position={"start"}>
              {Array.isArray(activeBond.bond.reward.asset) ? (
                <PairAsset assets={activeBond.bond.reward.asset} />
              ) : (
                <TokenAsset tokenId={activeBond.bond.reward.asset.id} />
              )}
            </InputAdornment>
          ),
        }}
      />
      <Button
        sx={{
          mt: theme.spacing(4),
        }}
        variant="contained"
        fullWidth
        disabled={claimable.eq(0)}
        onClick={handleClaim}
      >
        Claim {claimable.toString()}
      </Button>

      <Stack mt={theme.spacing(4)} width="100%">
        <PositionDetailsRow
          label="Pending reward"
          description={`${pending.toFormat(0)}`}
        />
        <PositionDetailsRow
          label="Claimable reward"
          description={`${humanBalance(claimable)}`}
        />
        <PositionDetailsRow
          label="Time until fully vested"
          description={`${vestingTime}`}
        />
        <PositionDetailsRow label="Vested" description={vestedTime} />
        <PositionDetailsRow
          label="ROI"
          description={`${humanBalance(
            getROI(activeBond.bond.rewardPrice, activeBond.bond.price)
          )}%`}
        />
      </Stack>
    </Box>
  );
};
