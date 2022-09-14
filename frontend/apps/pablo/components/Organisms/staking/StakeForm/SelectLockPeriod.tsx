import { alpha, Box, Button, Grid, Typography, useTheme } from "@mui/material";
import { Alert, Label } from "@/components/Atoms";
import { BoxProps } from "@mui/material";
import BigNumber from "bignumber.js";
import moment from "moment";
import { useMemo } from "react";

type Period = {
  days?: number;
  years?: number;
  months?: number;
  weeks?: number;
};

export type SelectLockPeriodProps = {
  periodItems: {
    label: string;
    period: Period;
    multiplier: BigNumber;
    value: string;
  }[];
  durationPreset: string;
  setDurationPreset?: (multiplier: string) => void;
} & BoxProps;

export const SelectLockPeriod: React.FC<SelectLockPeriodProps> = ({
  periodItems,
  durationPreset,
  setDurationPreset,
  ...boxProps
}) => {
  const theme = useTheme();

  const selected = (duration: string) => {
    return duration === durationPreset;
  };

  const expiry = useMemo(() => {
    if (durationPreset === "0") return null;
    const right_now = moment();
    right_now.add(durationPreset, "seconds");
    return right_now.format("DD.MM.YYYY");
  }, [durationPreset]);

  return (
    <Box {...boxProps}>
      <Label
        label="Select lock period (multiplier)"
        TypographyProps={{ color: "text.secondary" }}
        TooltipProps={{
          title: "Select lock period (multiplier)",
        }}
      />
      <Grid container spacing={3}>
        {periodItems.map((item) => (
          <Grid item sm={12} md={6} lg={3} key={item.multiplier.toString()}>
            <Button
              variant="outlined"
              size="large"
              fullWidth
              sx={{
                borderWidth: selected(item.value) ? 1 : 0,
                background: selected(item.value)
                  ? alpha(
                      theme.palette.primary.main,
                      theme.custom.opacity.light
                    )
                  : alpha(
                      theme.palette.common.white,
                      theme.custom.opacity.lighter
                    ),
              }}
              onClick={() => setDurationPreset?.(item.value)}
            >
              <Typography
                variant="body1"
                color={selected(item.value) ? "text.primary" : "text.secondary"}
              >
                {item.label}
              </Typography>
            </Button>
          </Grid>
        ))}
      </Grid>

      <Label
        mt={3}
        label="Unlock date"
        TypographyProps={{ color: "text.secondary" }}
        TooltipProps={{
          title: "Unlock date",
        }}
      />

      <Box
        py={2.25}
        borderRadius={9999}
        textAlign="center"
        sx={{
          background: theme.palette.gradient.secondary,
        }}
      >
        <Typography
          variant="body1"
          color={expiry ? "text.primary" : "text.secondary"}
        >
          {expiry !== null ? expiry : "Select lock time"}
        </Typography>
      </Box>

      {expiry && (
        <Box mt={1.5}>
          <Alert
            severity="warning"
            alertTitle="Warning"
            alertText="Your PICA will be locked until the expiry date."
            AlertTextProps={{ color: "text.secondary" }}
          />
        </Box>
      )}
    </Box>
  );
};
