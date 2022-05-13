import type { NextPage } from "next";
import {
  Container,
  Box,
  Grid,
  useTheme,
  Typography,
  alpha,
  Card,
} from "@mui/material";
import Default from "@/components/Templates/Default";
import { Chart, PageTitle } from "@/components";
import { ConnectWalletFeaturedBox } from "@/components/Organisms/ConnectWalletFeaturedBox";
import { Link } from "@/components";
import { YourBondTable } from "@/components/Organisms/YourBondTable";
import { AllBondTable } from "@/components/Organisms/AllBondTable";
import { useDotSamaContext } from "substrate-react";
import { DEFI_CONFIG } from "@/defi/config";
import { useState } from "react";

const standardPageSize = {
  xs: 12,
};

const twoColumnPageSize = {
  xs: 12,
  md: 6,
};

const Bond: NextPage = () => {
  const { extensionStatus } = useDotSamaContext();

  const theme = useTheme();
  const intervals = DEFI_CONFIG.bondChartIntervals;
  const [currentIntervalSymbolTVL, setCurrentIntervalSymbolTVL] = useState(
    intervals[0].symbol
  );
  const [currentIntervalSymbolVol, setCurrentIntervalSymbolVol] = useState(
    intervals[0].symbol
  );

  const onIntervalChange = (interval: string, chartName: "TVL" | "VOLUME") => {
    console.log(interval);
    chartName === "TVL"
      ? setCurrentIntervalSymbolTVL(interval)
      : setCurrentIntervalSymbolVol(interval);
  };

  const getCurrentInterval = (chartName: "TVL" | "VOLUME") => {
    return intervals.find((interval) => {
        const symbol = chartName === 'TVL' ? currentIntervalSymbolTVL : currentIntervalSymbolVol;
        return interval.symbol === symbol;
      });
  };

  return (
    <Default>
      <Container maxWidth="lg">
        <Box display="flex" flexDirection="column" alignItems="center" mb={8}>
          <PageTitle title="Bond" subtitle="Something about earning PICA" />
        </Box>
        {extensionStatus!=="connected" && (
          <Grid item {...standardPageSize}>
            <ConnectWalletFeaturedBox />
          </Grid>
        )}
        {extensionStatus==="connected" && (
          <Grid mt={4}>
            <Grid item {...standardPageSize}>
              <Card variant="outlined">
                <Box
                  display="flex"
                  mb={3}
                  justifyContent="space-between"
                  alignItems="center"
                >
                  <Typography variant="h6">Your liquidity</Typography>
                </Box>
                <YourBondTable />
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
        <Grid mt={3} container spacing={6}>
          <Grid item {...twoColumnPageSize}>
            <Chart
              title="TVL"
              changeTextColor={theme.palette.common.white}
              changeText={`Past ${getCurrentInterval("TVL")?.name ?? ""}`}
              AreaChartProps={{
                data: [
                  [1644550600000, 20],
                  [1644560620928, 45],
                  [1644570600000, 40],
                  [1644590600000, 100],
                ],
                height: 300,
                shorthandLabel: "Change",
                labelFormat: (n: number) => n.toFixed(),
                color: theme.palette.common.white,
              }}
              onIntervalChange={(value: string) =>
                onIntervalChange(value, "TVL")
              }
              intervals={intervals.map((interval) => interval.symbol)}
              currentInterval={currentIntervalSymbolTVL}
            />
          </Grid>
          <Grid item {...twoColumnPageSize}>
            <Chart
              title="Volume"
              changeTextColor={theme.palette.featured.main}
              changeText="+2% KSM"
              changeIntroText="Feb 8, '22"
              AreaChartProps={{
                data: [
                  [1644550600000, 20],
                  [1644560620928, 45],
                  [1644570600000, 40],
                  [1644590600000, 100],
                ],
                height: 300,
                shorthandLabel: "Change",
                labelFormat: (n: number) => n.toFixed(),
                color: theme.palette.featured.main,
              }}
              onIntervalChange={(value: string) =>
                onIntervalChange(value, "VOLUME")
              }
              intervals={intervals.map((interval) => interval.symbol)}
              currentInterval={currentIntervalSymbolVol}
            />
          </Grid>
        </Grid>
        <Grid mt={8}>
          <Grid item {...standardPageSize}>
            <Card variant="outlined">
              <Typography variant="h6" mb={2}>
                All bonds
              </Typography>
              <AllBondTable />
            </Card>
          </Grid>
        </Grid>
      </Container>
    </Default>
  );
};

export default Bond;
