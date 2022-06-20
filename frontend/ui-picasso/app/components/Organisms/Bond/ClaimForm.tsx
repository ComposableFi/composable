import { alpha, useTheme } from "@mui/material/styles";
import { Box, Button, InputAdornment, Stack, Typography } from "@mui/material";
import { BigNumberInput, TokenAsset } from "@/components";
import {
  useBlockInterval,
  usePicassoProvider,
  useSelectedAccount,
} from "@/defi/polkadot/hooks";
import { useOpenPositions } from "@/defi/polkadot/hooks/useOpenPositions";
import { useAppSelector } from "@/hooks/store";
import PositionDetailsRow from "@/components/Atom/PositionDetailsRow";
import { claim, fromPica, getROI } from "@/defi/polkadot/pallets/BondedFinance";
import BigNumber from "bignumber.js";
import { secondsToDHMS } from "@/defi/polkadot/hooks/useBondVestingInDays";
import router from "next/router";
import { ActiveBond } from "@/stores/defi/polkadot/bonds/slice";
import { getClaimable } from "@/components/Organisms/Bond/utils";
import { useCurrentBlockAndTime } from "@/defi/polkadot/utils";
import { PairAsset } from "@/components/Atom/PairAsset";
import { useExecutor } from "substrate-react";

function findCurrentBond(b: ActiveBond, bond: string): boolean {
  console.log(b);
  return b.bond.bondOfferId.toString() === bond;
}

export const ClaimForm = () => {
  const theme = useTheme();
  const account = useSelectedAccount();
  const { parachainApi } = usePicassoProvider();
  const { block } = useCurrentBlockAndTime(parachainApi);
  useOpenPositions(account);
  const executor = useExecutor();
  const interval = useBlockInterval();
  const { bond } = router.query;
  const openBonds = useAppSelector<ActiveBond[]>(
    (state) => state.bonding.openPositions
  );
  const activeBond = openBonds.find((b: ActiveBond) =>
    findCurrentBond(b, bond?.toString() ?? "")
  );
  if (activeBond === undefined || !parachainApi) return null;

  const handleClaim = () => {
    claim(
      {
        parachainApi,
        account,
        executor,
        assetId: activeBond.bond.reward.assetId,
      },
      () => {
        console.log("Successfully claimed");
      },
      () => {
        console.log("No success here");
      },
      () => {
        console.log("Started transactuin");
      }
    );
  };
  const { perPeriod, periodCount, window } = activeBond;
  const lastBlock = window.blockNumberBased.start
    .plus(window.blockNumberBased.period)
    .multipliedBy(periodCount);
  const claimable = getClaimable(
    block,
    window,
    perPeriod,
    lastBlock,
    periodCount
  );

  const total = periodCount.multipliedBy(fromPica(perPeriod));

  const pending = total.minus(claimable);
  const remainingBlocks = lastBlock.minus(block).lte(0)
    ? new BigNumber(0)
    : lastBlock.minus(block);
  const remainingTime = secondsToDHMS(
    remainingBlocks.multipliedBy(Number(interval) / 1000).toNumber()
  );
  const vesting_time = `${remainingTime.d
    .toString()
    .padStart(2, "00")}D${remainingTime.h
    .toString()
    .padStart(2, "00")}H${remainingTime.m
    .toString()
    .padStart(2, "00")}M${remainingTime.s.toString().padStart(2, "00")}S`;

  const vested_time = secondsToDHMS(
    block
      .minus(window.blockNumberBased.start)
      .multipliedBy(Number(interval) / 1000)
      .toNumber()
  );
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
        maxValue={new BigNumber(0)}
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
        onClick={handleClaim}
      >
        Claim
      </Button>

      <Stack mt={theme.spacing(4)}>
        <PositionDetailsRow
          label="Pending reward"
          description={`${pending.toFormat(0)}`}
        />
        <PositionDetailsRow
          label="Claimable reward"
          description={`${claimable.toFormat(0)}`}
        />
        <PositionDetailsRow
          label="Time until fully vested"
          description={`${vesting_time}`}
        />
        <PositionDetailsRow label="Vested" description={`${vested_time.d}`} />
        <PositionDetailsRow
          label="ROI"
          description={`${getROI(
            activeBond.bond.rewardPrice,
            activeBond.bond.bondPrice
          ).toFormat(0)}%`}
        />
      </Stack>
    </Box>
  );
};
