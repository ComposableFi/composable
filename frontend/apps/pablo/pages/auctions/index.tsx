import type { NextPage } from "next";
import {
  alpha,
  Box,
  Button,
  Container,
  Grid,
  Tooltip,
  Typography,
  useTheme,
} from "@mui/material";
import Default from "@/components/Templates/Default";
import { PageTitle } from "@/components";
import { AllAuctionsTable } from "@/components/Organisms/AllAuctionsTable";
import { useEffect, useState } from "react";
import { useDotSamaContext } from "substrate-react";
import { HighlightBox } from "@/components/Atoms/HighlightBox";
import { usePoolsSlice } from "@/store/pools/pools.slice";
import { setAuctionsSpotPrice } from "@/store/auctions/auctions.slice";

const standardPageSize = {
  xs: 12,
};

const Auctions: NextPage = () => {
  const theme = useTheme();
  const [enabledCreate] = useState<boolean>(false);
  const { extensionStatus } = useDotSamaContext();
  const {
    liquidityBootstrappingPools
  } = usePoolsSlice();
  useEffect(() => {
    if (liquidityBootstrappingPools.length > 0) {
      const interval = setInterval(() => {
        for (const pool of liquidityBootstrappingPools) {
          const poolId = pool.getPoolId() as string;
          pool.getSpotPrice().then(spotPrice => {
            setAuctionsSpotPrice(poolId, spotPrice)
          })
        }
      }, 1000 * 60);

      return () => clearInterval(interval);
    }
  }, [
    liquidityBootstrappingPools,
  ]);

  return (
    <Default>
      <Container maxWidth="lg">
        <PageTitle
          title="Token Launch Auctions"
          subtitle="Liquidity Bootstrapping Pools for Pablo"
        />
        <Box mb={25}>
          <Grid container mt={4}>
            <Grid item {...standardPageSize}>
              <HighlightBox>
                <Box
                  display="flex"
                  mb={4}
                  justifyContent="space-between"
                  alignItems="center"
                >
                  <Typography variant="h6">All liquidity</Typography>
                  <Box>
                    <Tooltip
                      title={
                        extensionStatus !== "connected" ? "Coming soon" : ""
                      }
                      arrow
                    >
                      {enabledCreate ? (
                        <Button
                          onClick={() => {}}
                          variant="contained"
                          size="small"
                          disabled
                        >
                          Create auction
                        </Button>
                      ) : (
                        <Box
                          sx={{
                            padding: theme.spacing(1.5, 3),
                            background: alpha(
                              theme.palette.primary.main,
                              theme.custom.opacity.main
                            ),
                            borderRadius: 9999,
                          }}
                        >
                          <Typography variant="button" color="text.secondary">
                            Create auction
                          </Typography>
                        </Box>
                      )}
                    </Tooltip>
                  </Box>
                </Box>
                <AllAuctionsTable />
              </HighlightBox>
            </Grid>
          </Grid>
        </Box>
      </Container>
    </Default>
  );
};

export default Auctions;
