import { Grid, Typography } from "@mui/material";
import { HighlightBox } from "@/components/Atoms/HighlightBox";
import { useParachainApi } from "substrate-react";
import useStore from "@/store/useStore";
import { useEffect } from "react";
import { PoolsTable } from "@/components/Organisms/PoolsTable";
import { subscribePools } from "@/store/pools/subscribePools";

export const AllLiquidityTable = () => {
  const { parachainApi } = useParachainApi("picasso");
  const poolsConfig = useStore((state) => state.pools.config);

  useEffect(() => {
    if (parachainApi) {
      return subscribePools(parachainApi);
    }
  }, [parachainApi]);

  return (
    <Grid mt={4}>
      <Grid item xs={12}>
        <HighlightBox textAlign="left">
          <Typography variant="h6" mb={2}>
            All Liquidity
          </Typography>
          <PoolsTable liquidityPools={poolsConfig} source="pallet" />
        </HighlightBox>
      </Grid>
    </Grid>
  );
};
