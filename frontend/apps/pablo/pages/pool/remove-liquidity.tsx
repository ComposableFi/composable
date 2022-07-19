import type { NextPage } from "next";
import {
  Container,
  Typography,
  Box,
} from "@mui/material";
import Default from "@/components/Templates/Default";
import { Link } from "@/components";
import { RemoveLiquidityForm } from "@/components/Organisms/liquidity/RemoveForm";
import { useEffect } from "react";
import { useRouter } from "next/router";
import useStore from "@/store/useStore";
import { useDotSamaContext } from "substrate-react";

const RemoveLiquidity: NextPage = () => {

  const breadcrumbs = [
    <Link key="pool" underline="none" color="primary" href="/pool">
      Pool
    </Link>,
    <Typography key="addliquidity" color="text.primary">
      Remove liquidity
    </Typography>,
  ];

  const {extensionStatus} = useDotSamaContext();
  const { ui: { isPolkadotModalOpen } } = useStore();
  const router = useRouter();

  useEffect(() => {
    extensionStatus !== "connected" && !isPolkadotModalOpen && router.push('/pool');
  });

  return (
    <Default breadcrumbs={breadcrumbs}>
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
          <RemoveLiquidityForm />
        </Box>
      </Container>
    </Default>

  );
};

export default RemoveLiquidity;
