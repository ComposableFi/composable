import {
  alpha,
  Box,
  Button,
  Grid,
  GridProps,
  Typography,
  useTheme,
} from "@mui/material";
import { BoxWrapper } from "../../BoxWrapper";
import { DonutChart } from "@/components/Atoms/DonutChart";
import { PoolDetailsProps } from "./index";
import { useRemoveLiquidityState } from "@/store/removeLiquidity/hooks";
import { useRouter } from "next/router";

const twoColumnPageSize = {
  sm: 12,
  md: 6,
};

type ItemProps = {
  value: string;
} & GridProps;

const Item: React.FC<ItemProps> = ({ value, children, ...gridProps }) => {
  return (
    <Grid container {...gridProps}>
      <Grid item {...twoColumnPageSize}>
        {children}
      </Grid>
      <Grid item {...twoColumnPageSize} textAlign="right">
        <Typography variant="subtitle1" fontWeight={600}>
          {value}
        </Typography>
      </Grid>
    </Grid>
  );
};

const DonutChartLabels = ["My Position", "Total Value Locked"];

export const PoolLiquidityPanel: React.FC<PoolDetailsProps> = ({
  poolId,
  ...boxProps
}) => {
  const router = useRouter();
  const theme = useTheme();
  const { setRemoveLiquidity } = useRemoveLiquidityState();
  const handleAddLiquidity = () => {
    router.push("/pool/add-liquidity");
  };
  const handleRemoveLiquidity = () => {
    router.push("/pool/remove-liquidity");
  };

  return (
    <BoxWrapper {...boxProps}>
      <Grid container>
        <Grid item {...twoColumnPageSize}>
          <Typography variant="h5">{`N/A`}</Typography>
          <Typography variant="body1" color="text.secondary">
            Liquidity Provided
          </Typography>
        </Grid>
        <Grid container item {...twoColumnPageSize} spacing={3}>
          <Grid item {...twoColumnPageSize}>
            <Button
              variant="contained"
              size="large"
              fullWidth
              onClick={handleAddLiquidity}
            >
              Add liquidity
            </Button>
          </Grid>
          <Grid item {...twoColumnPageSize}>
            <Button
              disabled={false}
              variant="outlined"
              size="large"
              fullWidth
              onClick={handleRemoveLiquidity}
            >
              Remove liquidity
            </Button>
          </Grid>
        </Grid>
      </Grid>

      <Box mt={4}>
        <Grid container spacing={4}>
          <Grid item {...twoColumnPageSize}>
            <DonutChart
              data={[20, 80]}
              colors={[
                alpha(theme.palette.common.white, theme.custom.opacity.main),
                theme.palette.primary.main,
              ]}
              labels={DonutChartLabels}
              height={"249px"}
            />
          </Grid>
          <Grid item {...twoColumnPageSize}>
            <Box mt={8}></Box>
          </Grid>
        </Grid>
      </Box>
    </BoxWrapper>
  );
};
