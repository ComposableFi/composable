import type { NextPage } from "next";
import {
  alpha,
  Box,
  Button,
  Container,
  Grid,
  Typography,
  useTheme,
} from "@mui/material";
import { useState } from "react";
import { useRouter } from "next/router";
import { MessageBox } from "@/components/Atoms";
import { Link, PageTitle } from "@/components";
import { ConnectWalletFeaturedBox } from "@/components/Organisms/ConnectWalletFeaturedBox";
import { useDotSamaContext } from "substrate-react";
import { resetAddLiquiditySlice } from "@/store/addLiquidity/addLiquidity.slice";
import { HighlightBox } from "@/components/Atoms/HighlightBox";
import { useAllLpTokenRewardingPools } from "@/defi/hooks/useAllLpTokenRewardingPools";
import { PoolsTable } from "@/components/Organisms/PoolsTable";
import { usePoolsWithLpBalance } from "@/defi/hooks/overview/usePoolsWithLpBalance";
import Default from "@/components/Templates/Default";
import useStore from "@/store/useStore";

const standardPageSize = {
  xs: 12,
};

const Pool: NextPage = () => {
  const { extensionStatus } = useDotSamaContext();

  const { createPool } = useStore();
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

  const allLpRewardingPools = useAllLpTokenRewardingPools();
  const poolsWithUserProvidedLiquidity = usePoolsWithLpBalance();

  return (
    <Default>
      <Container maxWidth="lg">
        <PageTitle title="Pool" subtitle="Add liquidity to earn tokens." />
        <Box display="flex" flexDirection="column" alignItems="center" mb={8}>
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
        {extensionStatus !== "connected" && (
          <Grid item {...standardPageSize}>
            <ConnectWalletFeaturedBox />
          </Grid>
        )}
        {extensionStatus === "connected" && (
          <Grid>
            <Grid item {...standardPageSize}>
              <HighlightBox>
                <Box
                  display="flex"
                  mb={3}
                  justifyContent="space-between"
                  alignItems="center"
                >
                  <Typography variant="h6">Your Liquidity</Typography>
                  <Box>
                    <Button
                      disabled
                      sx={{ marginRight: 2 }}
                      onClick={handleCreatePair}
                      variant="outlined"
                    >
                      Create a pair
                    </Button>
                    <Button disabled={
                      allLpRewardingPools.length === 0
                    } onClick={handleClick} variant="contained">
                      Add Liquidity
                    </Button>
                  </Box>
                </Box>
                {
                  <PoolsTable liquidityPools={poolsWithUserProvidedLiquidity} source="user" />
                }
              </HighlightBox>
              <Box mt={4} display="none" gap={1} justifyContent="center">
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
                <Link href="/pool/import" key="import">
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
            <HighlightBox textAlign="left">
              <Typography variant="h6" mb={2}>
                All Liquidity
              </Typography>
              {
                <PoolsTable liquidityPools={allLpRewardingPools} source="pallet" />
              }
            </HighlightBox>
          </Grid>
        </Grid>
      </Container>
    </Default>
  );
};

export default Pool;
