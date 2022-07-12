import type { NextPage } from "next";
import {
  Container,
  Box,
  Grid,
  useTheme,
  Typography,
  alpha,
  Button,
  Card,
} from "@mui/material";
import { useState } from "react";
import { useRouter } from "next/router";
import Default from "@/components/Templates/Default";
import { MessageBox } from "@/components/Atoms";
import { PageTitle } from "@/components";
import { ConnectWalletFeaturedBox } from "@/components/Organisms/ConnectWalletFeaturedBox";
import { AllLiquidityTable } from "@/components/Organisms/AllLiquidityTable";

import { Link } from "@/components";
import {useDotSamaContext} from "substrate-react";
import { resetAddLiquiditySlice } from "@/store/addLiquidity/addLiquidity.slice";
import useStore from "@/store/useStore";


const standardPageSize = {
  xs: 12,
};

const twoColumnPageSize = {
  xs: 12,
  md: 6,
};

const Pool: NextPage = () => {
  const {extensionStatus} = useDotSamaContext();

  const {createPool} = useStore();
  const theme = useTheme();
  const router = useRouter();
  const [messageBoxOpen, setMessageBoxOpen] = useState(true);

  const handleClick = () => {
    resetAddLiquiditySlice();
    router.push("/pool/add-liquidity");
  };

  const handleCreatePair = () => {
    createPool.resetSlice();
    router.push("/pool/create-pool");
  };

  return (
    <Default>
      <Container maxWidth="lg">
        <Box display="flex" flexDirection="column" alignItems="center" mb={8}>
          <PageTitle title="Pool" subtitle="Add liquidity to earn tokens." />
          {messageBoxOpen && (
            <MessageBox
              title="Liquidity provider rewards"
              message="Liquidity providers earn a 0.3% fee (default for all pairs, subject
            to change) on all trades proportional to their share of the pool.
            Fees are added to the pool, accrue in real time and can be claimed
            by withdrawing your liquidity."
              onClose={() => setMessageBoxOpen(false)}
            />
          )}
        </Box>
        {extensionStatus!=="connected" && (
          <Grid item {...standardPageSize}>
            <ConnectWalletFeaturedBox />
          </Grid>
        )}
        {extensionStatus==="connected" && (
          <Grid>
            <Grid item {...standardPageSize}>
              <Card variant="outlined">
                <Box
                  display="flex"
                  mb={3}
                  justifyContent="space-between"
                  alignItems="center"
                >
                  <Typography variant="h6">Your Liquidity</Typography>
                  <Box>
                    <Button
                      sx={{ marginRight: 2 }}
                      onClick={handleCreatePair}
                      variant="outlined"
                    >
                      Create a pair
                    </Button>
                    <Button onClick={handleClick} variant="contained">
                      Add Liquidity
                    </Button>
                  </Box>
                </Box>
                <AllLiquidityTable flow="user" />
              </Card>
              <Box mt={4} display="flex" gap={1} justifyContent="center">
                <Typography
                  textAlign="center"
                  variant="body2"
                  color={alpha(
                    theme.palette.common.white,
                    theme.custom.opacity.darker
                  )}
                >
                  {`Don't see a pool you joined?`}
                </Typography>
                <Link href="pool/import" key="import">
                  <Typography
                    textAlign="center"
                    variant="body2"
                    color="primary"
                    sx={{ cursor: "pointer" }}
                  >
                    Import it.
                  </Typography>
                </Link>
              </Box>
            </Grid>
          </Grid>
        )}
        <Grid mt={4}>
          <Grid item {...standardPageSize}>
            <Card variant="outlined">
              <Typography variant="h6" mb={2}>
                All Liquidity
              </Typography>
              <AllLiquidityTable flow="all" />
            </Card>
          </Grid>
        </Grid>
      </Container>
    </Default>
  );
};

export default Pool;
