import { Modal, TokenAsset } from "@/components";
import { FC, useMemo, useState } from "react";
import {
  alpha,
  Box,
  Button,
  Paper,
  Stack,
  Typography,
  useTheme,
} from "@mui/material";
import { formatDate, formatNumber } from "shared";
import { useStakingRewards } from "@/defi/polkadot/hooks/stakingRewards/useStakingRewards";
import { LockPeriodInput } from "@/components/Organisms/Staking/LockPeriodInput";
import {
  getMaxDuration,
  getMinDuration,
  getOptions,
} from "@/components/Organisms/Staking/utils";
import { FutureDatePaper } from "@/components/Atom/FutureDatePaper";
import { useExtendCall } from "@/defi/polkadot/hooks/stakingRewards/useExtendCall";
import { pipe } from "fp-ts/function";
import * as O from "fp-ts/Option";

export const RenewModal: FC<{
  open: boolean;
  onClose: () => void;
  selectedToken: [string, string];
}> = ({ open, onClose, selectedToken }) => {
  const theme = useTheme();
  const { stakingPortfolio, hasRewardPools, picaRewardPool } =
    useStakingRewards();
  const [isValid, setValid] = useState(true);
  const [fnftCollectionId, fnftInstanceId] = selectedToken;
  const [extendPeriod, setExtendPeriod] = useState("");

  // FORM Related info
  const options = getOptions(hasRewardPools, picaRewardPool);
  const minDuration = getMinDuration(hasRewardPools, picaRewardPool);
  const maxDuration = getMaxDuration(hasRewardPools, picaRewardPool);
  const extendCall = useExtendCall(onClose);

  const extend = () => {
    pipe(
      extendCall(),
      O.map((extend) =>
        extend(Number(extendPeriod), fnftCollectionId, fnftInstanceId)
      )
    );
  };

  // TODO: Refactor finding of current portfolio to a shared place
  const currentPortfolio = stakingPortfolio.find(
    (portfolio) =>
      portfolio.collectionId === fnftCollectionId &&
      portfolio.instanceId === fnftInstanceId
  );

  const previousDate = useMemo(() => {
    return currentPortfolio?.endTimestamp
      ? new Date(Number(currentPortfolio.endTimestamp))
      : null;
  }, [currentPortfolio?.endTimestamp]);

  const previousUnlockDate = useMemo(() => {
    return previousDate ? formatDate(previousDate) : null;
  }, [previousDate]);

  if (!currentPortfolio) {
    return null;
  }

  const initialPicaDeposit = currentPortfolio.stake;

  return (
    <Modal open={open} dismissible onClose={onClose} maxWidth="md">
      <Stack gap={4}>
        <Typography variant="h5" textAlign="center" marginBottom={4}>
          Add to staking period
        </Typography>
        <Stack gap={8}>
          <Box>
            <Typography variant="inputLabel">Initial PICA deposit</Typography>
            <Paper
              sx={{
                position: "relative",
                backgroundColor: alpha(theme.palette.common.white, 0.02),
              }}
            >
              <Box
                sx={{
                  position: "absolute",
                  left: "1rem",
                  top: "50%",
                  transform: "translateY(-50%)",
                }}
              >
                <TokenAsset tokenId={"pica"} iconOnly />
              </Box>
              <Typography
                textAlign="center"
                variant="body2"
                color="text.secondary"
              >
                {formatNumber(initialPicaDeposit)}
              </Typography>
            </Paper>
          </Box>
          <Box>
            <LockPeriodInput
              options={options}
              picaRewardPool={picaRewardPool}
              duration={extendPeriod}
              hasRewardPools={hasRewardPools}
              min={minDuration}
              max={maxDuration}
              onChange={(_, value) => setExtendPeriod(String(value))}
              label="Extend your lock period by"
              LabelProps={{
                variant: "inputLabel",
              }}
            />
          </Box>
          <Box>
            <Stack direction="row" gap={4}>
              <Box
                flexGrow={1}
                display="flex"
                flexDirection={"column"}
                gap={1.5}
              >
                <Typography variant="inputLabel">
                  Previous unlock date
                </Typography>
                <Paper
                  sx={{
                    backgroundColor: alpha(theme.palette.common.white, 0.02),
                    color: theme.palette.text.secondary,
                    textAlign: "center",
                  }}
                >
                  {previousUnlockDate}
                </Paper>
              </Box>
              <Box
                flexGrow={1}
                display="flex"
                flexDirection={"column"}
                gap={1.5}
              >
                <Typography variant="inputLabel">New unlock date</Typography>
                <FutureDatePaper
                  duration={extendPeriod}
                  previousDate={previousDate ?? undefined}
                  PaperProps={{
                    sx: {
                      backgroundColor: alpha(theme.palette.common.white, 0.02),
                    },
                  }}
                />
              </Box>
            </Stack>
          </Box>
        </Stack>
        <Button
          disabled={extendPeriod === "" || extendPeriod === "0" || !isValid}
          variant="contained"
          color="primary"
          fullWidth
          onClick={extend}
        >
          Add stake to period
        </Button>
      </Stack>
    </Modal>
  );
};
