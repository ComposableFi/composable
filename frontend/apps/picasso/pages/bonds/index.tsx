import type { NextPage } from "next";
import { useRouter } from "next/router";
import Default from "@/components/Templates/Default";
import { alpha, Box, Grid, Typography, useTheme } from "@mui/material";
import { MyBondsTable, PageTitle } from "@/components";
import { AllBondsTable } from "@/components/Molecules/AllBondsTable";
import { Updater } from "@/stores/defi/polkadot/bonds/PolkadotBondsUpdater";
import { useActiveBonds } from "@/defi/polkadot/hooks/useActiveBonds";
import { useStore } from "@/stores/root";
import { DisconnectedBond } from "@/components/Organisms/Bond/DisconnectedBond";
import { useDotSamaContext } from "substrate-react";

const standardPageSize = {
  xs: 12
};

const Bonds: NextPage = () => {
  const { activeBonds } = useActiveBonds();
  const theme = useTheme();
  const router = useRouter();
  const { extensionStatus } = useDotSamaContext();
  const bonds = useStore((state) => state.bonds.bonds);
  const handleActiveBondsClick = (offerId: string) => {
    router.push({
      pathname: `/bonds/${offerId}`
    });
  };

  return (
    <Default>
      <Updater />
      <Box flexGrow={1} sx={{ mx: "auto" }} maxWidth={1032} paddingBottom={16}>
        <Grid container spacing={4}>
          <Grid item {...standardPageSize} mt={theme.spacing(9)}>
            <PageTitle
              title="xPICA Bonds"
              subtitle="Bond liquidity to purchase xPICA at a discount"
              textAlign="center"
            />
          </Grid>
          {extensionStatus !== "connected" ? (
            <DisconnectedBond />
          ) : (
            <>
              <Grid item {...standardPageSize}>
                <Box
                  padding={4}
                  borderRadius={1}
                  bgcolor={alpha(theme.palette.common.white, 0.02)}
                >
                  <Typography mb={2}>Your Active Bonds</Typography>
                  <MyBondsTable
                    activeBonds={activeBonds}
                    onRowClick={handleActiveBondsClick}
                  />
                </Box>
              </Grid>
              <Grid item {...standardPageSize}>
                <Box
                  padding={4}
                  borderRadius={1}
                  bgcolor={alpha(theme.palette.common.white, 0.02)}
                >
                  <Typography mb={2}>All Bonds</Typography>
                  <AllBondsTable
                    bonds={bonds}
                    onRowClick={handleActiveBondsClick}
                  />
                </Box>
              </Grid>
            </>
          )}
        </Grid>
      </Box>
    </Default>
  );
};

export default Bonds;
