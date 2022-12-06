import { AlertBox, BigNumberInput } from "@/components";
import { FutureDatePaper } from "@/components/Atom/FutureDatePaper";
import { TextWithTooltip } from "@/components/Molecules/TextWithTooltip";
import { WarningAmberRounded } from "@mui/icons-material";
import { alpha, Box, Button, Stack, Typography, useTheme } from "@mui/material";
import BigNumber from "bignumber.js";

export const MockedStakingTab = () => {
  const theme = useTheme();
  return (
    <Stack sx={{ marginTop: theme.spacing(9) }} gap={4}>
      <Stack gap={1.5}>
        <Box
          display="flex"
          width="100%"
          justifyContent="space-between"
          alignItems="center"
        >
          <Typography variant="inputLabel">Amount to lock</Typography>
          <Box display="flex" gap={1}>
            <Typography variant="inputLabel" color="text.secondary">
              Balance:
            </Typography>
            <Typography variant="inputLabel">0 PICA</Typography>
          </Box>
        </Box>
        <BigNumberInput
          isValid={() => {}}
          setter={() => {}}
          maxValue={new BigNumber(0)}
          value={new BigNumber(0)}
          tokenId={"pica"}
          maxDecimals={12}
        />
      </Stack>
      {/*  Radiobutton groups*/}
      <Box display="flex" justifyContent="space-between" alignItems="center">
        <TextWithTooltip tooltip={"Select lock period"} disabled={true}>
          Select lock period (multiplier)
        </TextWithTooltip>
        <Box display="flex" justifyContent="flex-end" alignItems="center">
          <Typography
            variant="body2"
            color={alpha(theme.palette.common.white, 0.6)}
          >
            APR
          </Typography>
        </Box>
      </Box>
      <TextWithTooltip tooltip="Unlock date" disabled={true}>
        Unlock date
      </TextWithTooltip>
      <FutureDatePaper duration={""} />
      <AlertBox status="warning" icon={<WarningAmberRounded color="warning" />}>
        <Typography variant="body2">Warning</Typography>
        <Typography variant="inputLabel" color="text.secondary">
          Your PICA will be locked until the expiry date.
        </Typography>
      </AlertBox>
      <Button
        fullWidth
        onClick={() => {}}
        variant="contained"
        color="primary"
        disabled={true}
      >
        <Typography variant="button">Lock and mint</Typography>
      </Button>
    </Stack>
  );
};
