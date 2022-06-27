import type { NextPage } from "next";
import Default from "@/components/Templates/Default";
import { Box, Grid, Typography, useTheme } from "@mui/material";
import {
  Chart,
  ConnectWalletFeaturedBox,
  FeaturedBox,
  Link,
  MyAssetsTable,
  PageTitle,
  TabItem,
  TabPanel,
  Tabs,
} from "@/components";
import { useAppSelector } from "@/hooks/store";
import Image from "next/image";
import { CrowdloanRewardsFeaturedBox } from "@/components/Organisms/CrowdloanRewards/CrowdloanRewardsFeaturedBox";
import { useContext, useState } from "react";
import { MyStakingsTable } from "@/components/Molecules/MyStakingsTable";
import { ParachainContext } from "@/defi/polkadot/context/ParachainContext";

const Overview: NextPage = () => {
  const { extensionStatus } = useContext(ParachainContext);
  const assets = useAppSelector((state) =>
    Object.values(state.substrateBalances)
  );
  const myStakings = useAppSelector((state) => state.polkadot.myStakingAssets);
  const myBondings: any = [];
  const tabs: TabItem[] = [
    { label: "My assets" },
    { label: "My stakings", disabled: false },
    { label: "My bondings", disabled: false },
  ];

  const [tabValue, setTabValue] = useState(0);

  const theme = useTheme();
  const standardPageSize = {
    xs: 12,
  };
  return (
    <Default>
      <Grid
        container
        sx={{ mx: "auto" }}
        maxWidth={1032}
        rowSpacing={9}
        columns={10}
        direction="column"
        justifyContent="center"
      >
        <Grid item {...standardPageSize} mt={theme.spacing(9)}>
          <PageTitle
            title="Overview"
            textAlign="center"
            subtitle="You will be able to check on your positions here."
          />
        </Grid>
        {extensionStatus !== "connected" && (
          <Grid item {...standardPageSize}>
            <ConnectWalletFeaturedBox />
          </Grid>
        )}
        <Grid item {...standardPageSize}>
          <CrowdloanRewardsFeaturedBox />
        </Grid>
        {extensionStatus === "connected" && (
          <Chart
            title="My portoflio"
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
              height: 118,
              shorthandLabel: "Change",
              labelFormat: (n: number) => n.toFixed(),
              color: theme.palette.featured.lemon,
            }}
            intervals={["1h", "24h", "1w", "1m", "1y"]}
            marginTop={9}
          />
        )}
        {extensionStatus === "connected" && (
          <Grid item {...standardPageSize}>
            <Tabs
              items={tabs}
              value={tabValue}
              onChange={(_e, value) => setTabValue(value)}
            />

            {/* My Assets Tab Pannels */}
            <TabPanel value={tabValue} index={0}>
              <MyAssetsTable assets={assets} />
            </TabPanel>

            {/* My Staking Tab Pannels */}
            <TabPanel value={tabValue} index={1}>
              <Box px={2}>
                <PageTitle title="Picasso" textAlign="left" fontSize={40} />
              </Box>
              <MyAssetsTable assets={assets} />
            </TabPanel>

            <TabPanel value={tabValue} index={1}>
              <Box marginBottom={4} padding={2}>
                <Image
                  src="/logo/logo-pablo.svg"
                  width="150"
                  height="40"
                  alt="Pablo logo"
                />
              </Box>
              <MyStakingsTable assets={myStakings.pablo} />
            </TabPanel>

            {/* My Bondings Tab Pannels */}
            <TabPanel value={tabValue} index={2}>
              <Box px={2}>
                <PageTitle title="Picasso" textAlign="left" fontSize={40} />
              </Box>
              {/*<MyBondingsTable assets={[]} />*/}
            </TabPanel>
            <TabPanel value={tabValue} index={2}>
              <Box marginBottom={4} padding={2}>
                <Image
                  src="/logo/logo-pablo.svg"
                  width="150"
                  height="40"
                  alt="Pablo logo"
                />
              </Box>
              {/*<MyBondingsTable assets={myBondings.pablo} />*/}
            </TabPanel>
          </Grid>
        )}
        <Grid item {...standardPageSize}>
          <Typography variant="h6" align="center">
            Picasso projects
          </Typography>
          <Box
            sx={{
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              mt: theme.spacing(4),
              gap: theme.spacing(5),
            }}
          >
            <Box>
              <Link
                href="https://angular.finance/"
                target="_blank"
                rel="noopener"
              >
                <Image
                  src="/logo/logo-angular.svg"
                  width="125"
                  height="48"
                  alt="Angular logo"
                />
              </Link>
            </Box>
            <Box>
              <Link
                href="https://pablo.exchange"
                target="_blank"
                rel="noopener"
              >
                <Image
                  src="/logo/logo-pablo.svg"
                  width="172"
                  height="45"
                  alt="Pablo logo"
                  css={{
                    mixBlendMode: "luminosity",
                  }}
                />
              </Link>
            </Box>
          </Box>
        </Grid>
        <Grid item {...standardPageSize}>
          <FeaturedBox
            title="Join the pallet revolution, powered by Picasso"
            textBelow="To help support pallet development, we have launched the Composable Grant Program."
            horizontalAligned
            sx={{
              padding: theme.spacing(6),
            }}
            ButtonProps={{
              label: "Apply here",
              onClick: () => {},
              variant: "outlined",
            }}
          />
        </Grid>
      </Grid>
    </Default>
  );
};

export default Overview;
