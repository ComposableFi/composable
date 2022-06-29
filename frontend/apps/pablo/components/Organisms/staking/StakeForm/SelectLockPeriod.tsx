import { alpha, Box, Button, Grid, Typography, useTheme } from "@mui/material";
import { Alert, Label } from "@/components/Atoms";
import { BoxProps } from "@mui/material";
import moment from "moment-timezone";
import { Multiplier } from "./index";

const periodItems = [
  {
    label: "2 weeks (0x)",
    period: {weeks: 2},
    multiplier: 0,
  },
  {
    label: "2 months (0.25x)",
    period: {months: 2},
    multiplier: 0.25,
  },
  {
    label: "1 year (0.5x)",
    period: {years: 1},
    multiplier: 0.5,
  },
  {
    label: "2 years (1x)",
    period: {years: 2},
    multiplier: 1,
  },
];

type Period = {years?: number, months?: number, weeks?: number};

export type SelectLockPeriodProps = {
  multiplier: Multiplier,
  setMultiplier?: (multiplier: Multiplier) => void,
} & BoxProps;

export const SelectLockPeriod: React.FC<SelectLockPeriodProps> = ({
  multiplier,
  setMultiplier,
  ...boxProps
}) => {
  const theme = useTheme();

  const handleSelectPeriod = (period: Period, vMultiplier: number) => {
    setMultiplier?.({
      value: vMultiplier,
      expiry: moment().add(period).utc().valueOf(),
    });
  };

  const selected = (vMultiplier: number) => vMultiplier === multiplier.value;

  return (
    <Box {...boxProps}>
      <Label
        label="Select lock period (multiplier)"
        TypographyProps={{color: "text.secondary"}}
        TooltipProps={{
          title: "Select lock period (multiplier)",
        }}
      />
      <Grid container spacing={3}>
        {
          periodItems.map((item) => (
            <Grid item sm={12} md={6} lg={3} key={item.multiplier}>
              <Button
                variant="outlined"
                size="large"
                fullWidth
                sx={{
                  borderWidth: selected(item.multiplier) ? 1 : 0,
                  background: (
                    selected(item.multiplier)
                      ? alpha(theme.palette.primary.main, theme.custom.opacity.light)
                      : alpha(theme.palette.common.white, theme.custom.opacity.lighter)
                  ),
                }}
                onClick={() => handleSelectPeriod(item.period, item.multiplier)}
              >
                <Typography
                  variant="body1"
                  color={selected(item.multiplier) ? "text.primary" : "text.secondary"}
                >
                  {item.label}
                </Typography>
              </Button>
            </Grid>
          ))
        }
      </Grid>

      <Label
        mt={3}
        label="Unlock date"
        TypographyProps={{color: "text.secondary"}}
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
          color={multiplier.expiry ? "text.primary" : "text.secondary"}
        >
          {
            multiplier.expiry
              ? moment(multiplier.expiry).utc().format("DD.MM.YYYY")
              : "Select lock time"
          }
        </Typography>
      </Box>

      {multiplier.expiry && (
        <Box mt={1.5}>
          <Alert
            severity="warning"
            alertTitle="Warning"
            alertText="Your PICA will be locked until the expiry date."
            AlertTextProps={{color: "text.secondary"}}
          />
        </Box>
      )}
    </Box>
  );
};
