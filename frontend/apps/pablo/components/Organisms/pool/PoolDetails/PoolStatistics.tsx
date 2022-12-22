import {
  alpha,
  Box,
  BoxProps,
  Grid,
  Typography,
  useTheme,
} from "@mui/material";
import { PoolDetailsProps } from "./index";
import { FC } from "react";

const twoColumnPageSize = {
  sm: 12,
  md: 6,
};

type ItemProps = {
  label: string;
  value?: string;
} & BoxProps;

const Item: FC<ItemProps> = ({ label, value, children, ...boxProps }) => {
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
      {...boxProps}
    >
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
};

export const PoolStatistics: FC<PoolDetailsProps> = ({ pool, ...boxProps }) => {
  return (
    <Box {...boxProps}>
      <Grid container spacing={4}>
        <Grid item {...twoColumnPageSize}>
          <Item label="Pool value" value={`N/A`} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="Rewards left" py={2}></Item>
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="Volume (24H)" value={`N/A`} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="Fees (24H)" value={`N/A`} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="APY" value={`N/A`} />
        </Grid>
        <Grid item {...twoColumnPageSize}>
          <Item label="Transactions (24H)" value={`N/A`} />
        </Grid>
      </Grid>
    </Box>
  );
};
