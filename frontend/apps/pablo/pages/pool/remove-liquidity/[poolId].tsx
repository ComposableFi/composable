import type { NextPage } from "next";
import { Box, Container, Typography } from "@mui/material";
import { Link } from "@/components";
import { RemoveLiquidityForm } from "@/components/Organisms/liquidity/RemoveForm";
import { useEffect } from "react";
import { useRouter } from "next/router";
import { useDotSamaContext } from "substrate-react";
import { useUiSlice } from "@/store/ui/ui.slice";
import { PoolLayout } from "@/components/Templates/pools/PoolLayout";
import { usePoolDetail } from "@/defi/hooks/pools/usePoolDetail";
import { RemoveLiquiditySkeleton } from "@/components/Templates/pools/RemoveLiquiditySkeleton";
import useStore from "@/store/useStore";

const RemoveLiquidity: NextPage = () => {
  const breadcrumbs = [
    <Link key="pool" underline="none" color="primary" href="/pool">
      Pool
    </Link>,
    <Typography key="add-liquidity" color="text.primary">
      Remove liquidity
    </Typography>,
  ];

  const { extensionStatus } = useDotSamaContext();
  const { isPolkadotModalOpen } = useUiSlice();
  const router = useRouter();
  const { pool } = usePoolDetail();
  const isPoolLoaded = useStore((store) => store.pools.isLoaded);

  useEffect(() => {
    extensionStatus !== "connected" &&
      !isPolkadotModalOpen &&
      router.push("/pool");
  });

  if (pool === null || !isPoolLoaded) {
    return (
      <PoolLayout breadcrumbs={breadcrumbs}>
        <RemoveLiquiditySkeleton />
      </PoolLayout>
    );
  }

  return (
    <PoolLayout breadcrumbs={breadcrumbs}>
      <Container maxWidth="lg">
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "center",
            alignItems: "center",
            marginBottom: 18,
          }}
        >
          <RemoveLiquidityForm pool={pool} />
        </Box>
      </Container>
    </PoolLayout>
  );
};

export default RemoveLiquidity;
