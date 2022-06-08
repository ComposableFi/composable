import {
  Box,
  useTheme,
  Typography,
  BoxProps,
  Grid,
  alpha,
} from "@mui/material";
import { useAppSelector } from "@/hooks/store";
import { BaseAsset } from "@/components/Atoms";
import { TOKENS } from "@/defi/Tokens";

const twoColumnPageSize = {
  sm: 12,
  md: 6,
};

type ItemProps = {
  label: string,
  value?: string,
} & BoxProps;

const Item: React.FC<ItemProps> = ({
  label,
  value,
  children,
  ...boxProps
}) => {
  const theme = useTheme();
  return (
    <Box
      py={3.5}
      borderRadius={1}
      textAlign="center"
      border={`1px solid ${alpha(
        theme.palette.common.white,
        theme.custom.opacity.light
      )}`}
      sx={{
        background: theme.palette.gradient.secondary,
      }}
      {...boxProps}>
      <Typography variant="body1" color="text.secondary">
        {label}
      </Typography>
      {value && (
        <Typography variant="h6" mt={0.5}>
          {value}
        </Typography>
      )}
      {children && children}
    </Box>
  );
}

export const PoolStatistics: React.FC<BoxProps> = ({
  ...boxProps
}) => {
  const {
    poolValue,
    rewardsLeft,
    volume,
    fee24h,
    apr,
    transactions24h,
  } = useAppSelector(
    (state) => state.pool.selectedPool
  );

  return (
    <Box {...boxProps}>
      <Grid container spacing={4}>
        <Grid item {...twoColumnPageSize}>
          <Item label="Pool value" value={`$${poolValue.toFormat()}`} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="Rewards left" py={2}>
            {rewardsLeft.map(({tokenId, value}) => (
              <BaseAsset
                key={tokenId}
                icon={TOKENS[tokenId].icon}
                label={value.toFormat()}
                justifyContent="center"
                mt={0.5}
              />
            ))}
          </Item>
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="Volume (24H)" value={`$${volume.toFormat()}`} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="Fees (24H)" value={`$${fee24h.toFormat()}`} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="APR" value={`${apr}%`} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="Transactions (24H)" value={`${transactions24h}`} />
        </Grid>
      </Grid>
    </Box>
  );
};

