import type { NextPage } from "next";
import { useRouter } from "next/router";
import Default from "@/components/Templates/Default";
import { useTheme, Grid, Box, Typography, alpha } from "@mui/material";
import {
  Chart,
  ConnectWalletFeaturedBox,
  MyBondingsTable,
  PageTitle,
} from "@/components";
import { useStore } from "@/stores/root";
import { ConnectToStakeCover } from "@/components/Molecules/ConnectToStakeCover";
import { AllBondsTable } from "@/components/Molecules/AllBondsTable";
import { AllBondsAsset, BondingAsset } from "@/stores/defi/polkadot";
import { useContext } from "react";
import { ParachainContext } from "@/defi/polkadot/context/ParachainContext";

const standardPageSize = {
  xs: 12,
};

const TreasuryBonding: NextPage = () => {
  const theme = useTheme();
  const { extensionStatus } = useContext(ParachainContext);
  const { myBondingAssets, allBonds } = useStore(({ polkadot }) => polkadot);
  const router = useRouter();

  const handleActiveBondsClick = (asset: BondingAsset) => {
    let token = asset.token.symbol;
    let toToken = asset.toToken.symbol;
    router.push({
      pathname: `/treasury/bonding/active/${token}-${toToken}`,
      query: { token, toToken },
    });
  };

  const handleAllBondsClick = (asset: AllBondsAsset) => {
    let token = asset.token.symbol;
    let toToken = asset.toToken.symbol;
    router.push({
      pathname: `/treasury/bonding/all/${token}-${toToken}`,
      query: { token, toToken },
    });
  };

  return (
    <Default>
      <Box flexGrow={1} sx={{ mx: "auto" }} maxWidth={1032} paddingBottom={16}>
        <Grid container spacing={4}>
          <Grid item {...standardPageSize} mt={theme.spacing(9)}>
            <PageTitle
              title="CHAOS Bonds"
              subtitle="Bond liquidity to purchase CHAOS at a discount"
              textAlign="center"
            />
          </Grid>
          {extensionStatus == "connected" ? (
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
                <Chart
                  title="My portfolio"
                  totalText="$24,587,298"
                  changeText="+34%"
                  changeTextColor={theme.palette.featured.lemon}
                  AreaChartProps={{
                    data: [
                      [1644550600000, 20],
                      [1644560620928, 40],
                      [1644570600000, 35],
                      [1644580600000, 60],
                      [1644590600000, 80],
                    ],
                    height: 200,
                    shorthandLabel: "Change",
                    labelFormat: (n: number) => n.toFixed(),
                    color: theme.palette.primary.main,
                  }}
                  intervals={["1h", "24h", "1w", "1m", "1y"]}
                />
              </Grid>
              <Grid item {...standardPageSize}>
                <Box
                  padding={4}
                  borderRadius={1}
                  bgcolor={alpha(theme.palette.common.white, 0.02)}
                >
                  <Typography mb={2}>Your Active Bonds</Typography>
                  <MyBondingsTable
                    assets={myBondingAssets.picasso}
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
                    assets={allBonds}
                    onRowClick={handleAllBondsClick}
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

export default TreasuryBonding;
