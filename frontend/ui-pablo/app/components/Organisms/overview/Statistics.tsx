import { useAppSelector } from "@/hooks/store";
import { Box, useTheme, alpha, Grid, Typography } from "@mui/material";
import { GridProps } from "@mui/system";

const threeColumnPageSize = {
  xs: 12,
  md: 4,
};

type ItemProps = {
  label: string;
  value: string;
}
const Item: React.FC<ItemProps> = ({
  label,
  value,
}) => {
  const theme = useTheme();
  return (
    <Box
      py={4}
      borderRadius={0.6666}
      textAlign="center"
      sx={{
        background: theme.palette.gradient.secondary,
        border: `1px solid ${alpha(theme.palette.common.white, theme.custom.opacity.light)}`
      }}
    >
      <Typography variant="body1" color="text.secondary">
        {label}
      </Typography>
      <Typography variant="h6" mt={0.5}>
        {value}
      </Typography>
    </Box>
  )
};

export const Statistics: React.FC<GridProps> = ({
  ...gridProps
}) => {
  const theme = useTheme();

  const {
    totalValueLocked,
    tradingVolume24hrs,
    pabloPrice,
  } = useAppSelector((state) => state.polkadot.overview);

  return (
    <Grid container spacing={8} {...gridProps}>
      <Grid item {...threeColumnPageSize}>
        <Item label="Total value locked" value={`$${totalValueLocked.toFormat()}`} />
      </Grid>
      <Grid item {...threeColumnPageSize}>
        <Item label="24h trading volume" value={`$${tradingVolume24hrs.toFormat()}`} />
      </Grid>
      <Grid item {...threeColumnPageSize}>
        <Item label="PABLO price" value={`$${pabloPrice.toFormat()}`} />
      </Grid>
    </Grid>
  )
};
