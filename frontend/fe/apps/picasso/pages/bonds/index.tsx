import type { NextPage } from "next";
import { useRouter } from "next/router";
import Default from "@/components/Templates/Default";
import { alpha, Box, Grid, Typography, useTheme } from "@mui/material";
import {
  ConnectWalletFeaturedBox,
  MyBondingsTable,
  PageTitle,
} from "@/components";
import { useAppSelector } from "@/hooks/store";
import { ConnectToStakeCover } from "@/components/Molecules/ConnectToStakeCover";
import { AllBondsTable } from "@/components/Molecules/AllBondsTable";
import { useContext } from "react";
import { ParachainContext } from "@/defi/polkadot/context/ParachainContext";
import { Updater } from "@/stores/defi/polkadot/bonds/PolkadotBondsUpdater";
import { useSelectedAccount } from "@/defi/polkadot/hooks";
import { useOpenPositions } from "@/defi/polkadot/hooks/useOpenPositions";

const standardPageSize = {
  xs: 12,
};

const Bonds: NextPage = () => {
  const theme = useTheme();
  const { extensionStatus } = useContext(ParachainContext);
  const bonds = useAppSelector((state) => state.bonding.bonds);
  const account = useSelectedAccount();
  useOpenPositions(account);
  const openPositions = useAppSelector((state) => state.bonding.openPositions);
  const router = useRouter();

  const handleActiveBondsClick = (offerId: string) => {
    router.push({
      pathname: `/bonds/${offerId}`,
    });
  };

  return (
    <Default>
      <Updater />
      <Box flexGrow={1} sx={{ mx: "auto" }} maxWidth={1032} paddingBottom={16}>
        <Grid container spacing={4}>
          <Grid item {...standardPageSize} mt={theme.spacing(9)}>
            <PageTitle
              title="CHAOS Bonds"
              subtitle="Bond liquidity to purchase CHAOS at a discount"
              textAlign="center"
            />
          </Grid>
          {extensionStatus !== "connected" ? (
            <>
              <Grid item {...standardPageSize}>
                <ConnectWalletFeaturedBox message="To start staking, wallet needs to be connected." />
              </Grid>
              <Grid item {...standardPageSize}>
                <ConnectToStakeCover message="Connect to check your active positions." />
              </Grid>
            </>
          ) : (
            <>
              <Grid item {...standardPageSize}>
                <Box
                  padding={4}
                  borderRadius={1}
                  bgcolor={alpha(theme.palette.common.white, 0.02)}
                >
                  <Typography mb={2}>Your Active Bonds</Typography>
                  <MyBondingsTable
                    openPositions={openPositions}
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
