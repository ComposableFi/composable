import {
  alpha,
  Box,
  BoxProps,
  Theme,
  Typography,
  useTheme,
} from "@mui/material";

const overlapBoxProps = (theme: Theme) => ({
  height: "100%",
  width: "100%",
  position: "absolute",
  sx: {
    bottom: 0,
    left: 0,
    right: 0,
    borderRadius: 1,
    backgroundColor: alpha(theme.palette.common.white, theme.custom.opacity.lightest),
    backdropFilter: "blur(8px)",
    padding: theme.spacing(4),
  },
  textAlign: "center",
} as const);

export type BuyFormOverlapProps = {
  isActive: boolean,
  isEnded: boolean,
  start_date: string,
} & BoxProps;

export const BuyFormOverlap: React.FC<BuyFormOverlapProps> = ({
  isActive,
  isEnded,
  start_date,
  ...boxProps
}) => {
  const theme = useTheme();

  if (isActive) {
    return null;
  };

  if (isEnded) {
    return (
      <Box
        {...overlapBoxProps(theme)}
        {...boxProps}
      >
        <Typography variant="subtitle1" fontWeight={600}>
          The LBP has ended
        </Typography>
        <Typography variant="body1" mt={1.5}>
          Check the lists for more
        </Typography>
        <Typography variant="body1">
          upcoming LBP.
        </Typography>
      </Box>
    );
  };

  return (
    <Box
      {...overlapBoxProps(theme)}
      {...boxProps}
    >
      <Typography variant="subtitle1" fontWeight={600}>
        The LBP has not started
      </Typography>
      <Typography variant="body1" mt={1.5}>
        The LBP starts in {start_date}.
      </Typography>
      <Typography variant="body1">
        Swapping will be enabling by the
      </Typography>
      <Typography variant="body1">
        LBP creator at start time.
      </Typography>
    </Box>
  );
}