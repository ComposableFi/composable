import type { NextPage } from "next";
import { Box, Container, Typography } from "@mui/material";
import { ConnectWalletFeaturedBox, Link, PageTitle } from "@/components";
import { PoolDetails } from "@/components/Organisms/pool/PoolDetails";
import { useDotSamaContext } from "substrate-react";
import useStore from "@/store/useStore";
import { PoolSelectSkeleton } from "@/components/Templates/pools/PoolSelectSkeleton";
import { usePoolDetail } from "@/defi/hooks/pools/usePoolDetail";
import { PoolLayout } from "@/components/Templates/pools/PoolLayout";

const isConnected = (status: string) => status === "connected";

const breadcrumbs = [
  <Link key="pool" underline="none" color="primary" href="/pool">
    Pool
  </Link>,
  <Typography key="create-pool" color="text.primary">
    Select
  </Typography>,
];

const PoolDetailsPage: NextPage = () => {
  const { extensionStatus } = useDotSamaContext();
  const isPoolConfigLoaded = useStore((store) => store.pools.isLoaded);
  const { pool } = usePoolDetail();

  if (!isPoolConfigLoaded || pool === null) {
    return (
      <PoolLayout>
        <PoolSelectSkeleton />
      </PoolLayout>
    );
  }

  return (
    <PoolLayout breadcrumbs={breadcrumbs}>
      <Container maxWidth="lg">
        <Box display="flex" flexDirection="column" alignItems="center" mb={8}>
          <PageTitle
            title={`${pool.config.assets
              .map((asset) => asset.getSymbol())
              .join("/")} Pool`}
            subtitle="Earn tokens while adding liquidity."
          />
        </Box>
        {isConnected(extensionStatus) ? (
          <PoolDetails pool={pool} />
        ) : (
          <ConnectWalletFeaturedBox />
        )}
      </Container>
    </PoolLayout>
  );
};

export default PoolDetailsPage;
