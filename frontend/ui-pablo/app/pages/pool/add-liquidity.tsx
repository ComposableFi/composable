import type { NextPage } from "next";
import { 
  Container, 
  Typography, 
  Box
} from "@mui/material";
import Default from "@/components/Templates/Default";
import { Link } from "@/components";
import { useRouter } from "next/router";
import { useEffect } from "react";
import useStore from "@/store/useStore";
import { useDotSamaContext } from "substrate-react";
import { AddLiquidityForm } from "@/components/Organisms/liquidity/AddForm";
import AddLiquidityUpdater from "@/updaters/addLiquidity/Updater";

const AddLiquidity: NextPage = () => {

  const router = useRouter();
  const {extensionStatus} = useDotSamaContext();
  const { ui: { isPolkadotModalOpen } } = useStore();

  const breadcrumbs = [
    <Link key="pool" underline="none" color="primary" href="/pool">
      Pool
    </Link>,
    <Typography key="addliquidity" color="text.primary">
      Add liquidity
    </Typography>,
  ];

  useEffect(() => {
    extensionStatus !== "connected" && !isPolkadotModalOpen && router.push('/pool');
  }, [extensionStatus, isPolkadotModalOpen, router]);

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
          <AddLiquidityUpdater />
          <AddLiquidityForm />
        </Box>
      </Container>
    </Default>

  );
};

export default AddLiquidity;
