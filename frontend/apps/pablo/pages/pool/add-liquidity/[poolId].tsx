import type { NextPage } from "next";
import { Box, Container, Typography } from "@mui/material";
import { Link } from "@/components";
import { useEffect } from "react";
import { useDotSamaContext } from "substrate-react";
import { AddLiquidityForm } from "@/components/Organisms/liquidity/AddForm";
import { useUiSlice } from "@/store/ui/ui.slice";
import { useRouter } from "next/router";
import { PoolLayout } from "@/components/Templates/pools/PoolLayout";

const breadcrumbs = [
  <Link key="pool" underline="none" color="primary" href="/pool">
    Pool
  </Link>,
  <Typography key="add-liquidity" color="text.primary">
    Add liquidity
  </Typography>,
];

const AddLiquidityPage: NextPage = () => {
  const router = useRouter();
  const { extensionStatus } = useDotSamaContext();
  const { isPolkadotModalOpen } = useUiSlice();

  useEffect(() => {
    extensionStatus !== "connected" &&
      !isPolkadotModalOpen &&
      router.push("/pool");
  }, [extensionStatus, isPolkadotModalOpen, router]);

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
          <AddLiquidityForm />
        </Box>
      </Container>
    </PoolLayout>
  );
};

export default AddLiquidityPage;
