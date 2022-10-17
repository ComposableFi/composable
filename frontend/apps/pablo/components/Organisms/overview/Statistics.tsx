import { useAppSelector } from "@/hooks/store";
import { Grid, Typography, useTheme } from "@mui/material";
import { GridProps } from "@mui/system";
import { HighlightBox } from "@/components/Atoms/HighlightBox";

const threeColumnPageSize = {
  xs: 12,
  md: 4,
};

type ItemProps = {
  label: string;
  value: string;
};
const Item: React.FC<ItemProps> = ({ label, value }) => {
  const theme = useTheme();
  return (
    <HighlightBox
      variant="contained"
      py={4}
      borderRadius={1}
      textAlign="center"
    >
      <Typography variant="body1" color="text.secondary">
        {label}
      </Typography>
      <Typography variant="h6" mt={0.5}>
        {value}
      </Typography>
    </HighlightBox>
  );
};

export const Statistics: React.FC<GridProps> = ({ ...gridProps }) => {
  const theme = useTheme();

  const { totalValueLocked, tradingVolume24hrs, pabloPrice } = useAppSelector(
    (state) => state.polkadot.overview
  );

  return (
    <Grid container spacing={3} {...gridProps}>
      <Grid item {...threeColumnPageSize}>
        <Item
          label="Total value locked"
          value={`$${totalValueLocked.toFormat()}`}
        />
      </Grid>
      <Grid item {...threeColumnPageSize}>
        <Item
          label="24h trading volume"
          value={`$${tradingVolume24hrs.toFormat()}`}
        />
      </Grid>
      <Grid item {...threeColumnPageSize}>
        <Item label="PABLO price" value={`$${pabloPrice.toFormat()}`} />
      </Grid>
    </Grid>
  );
};
