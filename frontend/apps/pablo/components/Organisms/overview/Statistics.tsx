import { Grid, Skeleton, Typography } from "@mui/material";
import { GridProps } from "@mui/system";
import { HighlightBox } from "@/components/Atoms/HighlightBox";
import { FC } from "react";
import { useStatsTVL } from "@/components/Organisms/overview/useStatsTVL";
import { useDailyVolume } from "@/components/Organisms/overview/useDailyVolume";

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
  const [totalValueLocked, statsLoading] = useStatsTVL();
  const [dailyVolume, isVolumeLoading] = useDailyVolume();

  return (
    <Grid container spacing={3} {...gridProps}>
      <Grid item {...twoColumns}>
        {statsLoading ? (
          <Skeleton width="100%" height="133px" variant="rounded" />
        ) : (
          <Item
            label="Total value locked"
            value={`$${totalValueLocked.toFormat(2)}`}
          />
        )}
      </Grid>
      <Grid item {...twoColumns}>
        {isVolumeLoading ? (
          <Skeleton width="100%" height="133px" variant="rounded" />
        ) : (
          <Item
            label="24h trading volume"
            value={`$${dailyVolume.toFormat(0)}`}
          />
        )}
      </Grid>
    </Grid>
  );
};
