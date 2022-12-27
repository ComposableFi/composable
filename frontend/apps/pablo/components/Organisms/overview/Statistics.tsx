import { Grid, Typography } from "@mui/material";
import { GridProps } from "@mui/system";
import { HighlightBox } from "@/components/Atoms/HighlightBox";
import BigNumber from "bignumber.js";
import { FC } from "react";

const twoColumns = {
  xs: 12,
  sm: 6,
};

type ItemProps = {
  label: string;
  value: string;
};
const Item: FC<ItemProps> = ({ label, value }) => {
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

export const Statistics: FC<GridProps> = ({ ...gridProps }) => {
  const totalValueLocked = new BigNumber(0);
  const tradingVolume24hrs = new BigNumber(0);

  return (
    <Grid container spacing={3} {...gridProps}>
      <Grid item {...twoColumns}>
        <Item
          label="Total value locked"
          value={`$${totalValueLocked.toFormat()}`}
        />
      </Grid>
      <Grid item {...twoColumns}>
        <Item
          label="24h trading volume"
          value={`$${tradingVolume24hrs.toFormat()}`}
        />
      </Grid>
    </Grid>
  );
};
