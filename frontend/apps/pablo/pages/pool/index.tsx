import type { NextPage } from "next";
import { Container, Grid } from "@mui/material";
import { PageTitle } from "@/components";
import { ConnectWalletFeaturedBox } from "@/components/Organisms/ConnectWalletFeaturedBox";
import { useDotSamaContext } from "substrate-react";
import { AllLiquidityTable } from "@/components/Templates/pools/AllLiquidityTable";
import { LiquidityRewardsMessage } from "@/components/Organisms/pool/LiquidityRewardsMessage";
import { YourLiquidity } from "@/components/Organisms/pool/YourLiquidity";
import { PoolLayout } from "@/components/Templates/pools/PoolLayout";

const standardPageSize = {
  xs: 12,
};

const Pool: NextPage = () => {
  const { extensionStatus } = useDotSamaContext();

  return (
    <PoolLayout>
      <Container maxWidth="lg">
        <PageTitle title="Pool" subtitle="Add liquidity to earn tokens." />
        <LiquidityRewardsMessage />
        {extensionStatus !== "connected" ? (
          <Grid item {...standardPageSize}>
            <ConnectWalletFeaturedBox />
          </Grid>
        ) : (
          <YourLiquidity />
        )}
        <AllLiquidityTable />
      </Container>
    </PoolLayout>
  );
};

export default Pool;
