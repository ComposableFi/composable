import type { NextPage } from "next";
import {
  Container,
  Box,
  Grid,
  useTheme,
  Typography,
  alpha,
  Button,
  Tooltip,
} from "@mui/material";
import Default from "@/components/Templates/Default";
import { PageTitle } from "@/components";
import { AllAuctionsTable } from "@/components/Organisms/AllAuctionsTable";
import { useEffect, useState } from "react";
import { useDotSamaContext, useParachainApi } from "substrate-react";
import { retrieveSpotPrice } from "@/store/updaters/swaps/utils";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";

const standardPageSize = {
  xs: 12,
};

const Auctions: NextPage = () => {

  const theme = useTheme();
  const [enabledCreate] = useState<boolean>(false);
  const { extensionStatus } = useDotSamaContext();
  const { parachainApi } = useParachainApi("picasso");
  const { liquidityBootstrappingPools, putLBPSpotPrice } = useStore();

  useEffect(() => {
    if (parachainApi && liquidityBootstrappingPools.list.length > 0) {
      const interval = setInterval(() => {
        for (
          let pool = 0;
          pool < liquidityBootstrappingPools.list.length;
          pool++
        ) {
          retrieveSpotPrice(
            parachainApi,
            liquidityBootstrappingPools.list[pool].pair,
            liquidityBootstrappingPools.list[pool].poolId
          ).then((spotPrice) => {
            putLBPSpotPrice(spotPrice.toFixed(4), pool);
          });
        }
      }, 1000 * 60);

      return () => clearInterval(interval)
    }
  }, [parachainApi, liquidityBootstrappingPools.list.length]);

  return (
    <Default>
      
      <Container maxWidth="lg">
        <Box mb={25}>
          <Box display="flex" flexDirection="column" alignItems="center" mb={8}>
            <PageTitle
              title="Token Launch Auctions"
              subtitle="Liquidity Bootstrapping Pools for Pablo"
            />
          </Box>
          <Grid container mt={4}>
            <Grid item {...standardPageSize}>
              <Box
                padding={4}
                sx={{
                  background: theme.palette.gradient.secondary,
                  borderRadius: 1,
                }}
                border={`1px solid ${alpha(
                  theme.palette.common.white,
                  theme.custom.opacity.light
                )}`}
              >
                <Box
                  display="flex"
                  mb={4}
                  justifyContent="space-between"
                  alignItems="center"
                >
                  <Typography variant="h6">All liquidity</Typography>
                  <Box>
                    <Tooltip 
                      title={extensionStatus !== "connected" ? "Comming soon" : ""}
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
                            background: alpha(theme.palette.primary.main, theme.custom.opacity.main),
                            borderRadius: 9999,
                          }}
                        >
                          <Typography
                            variant="button"
                            color="text.secondary"
                          >
                            Create auction
                          </Typography>
                        </Box>
                      )}
                    </Tooltip>
                  </Box>
                </Box>
                <AllAuctionsTable />
              </Box>
            </Grid>
          </Grid>
        </Box>
      </Container>
    </Default>
  );
};

export default Auctions;
