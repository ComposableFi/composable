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
import Image from "next/image";
import { CrowdloanRewardsFeaturedBox } from "@/components/Organisms/CrowdloanRewards/CrowdloanRewardsFeaturedBox";
import { useState } from "react";
import { MyStakesTable } from "@/components/Molecules/MyStakesTable";
import { useStore } from "@/stores/root";
import { useDotSamaContext } from "substrate-react";

const Overview: NextPage = () => {
  const { extensionStatus } = useDotSamaContext();
  const myStakes = useStore((state) => state.polkadot.myStakingAssets);
  const tabs: TabItem[] = [
    { label: "My assets" },
    { label: "My stakes", disabled: true },
    { label: "My bonds", disabled: true },
  ];

  const [tabValue, setTabValue] = useState(0);

  const theme = useTheme();
  const standardPageSize = {
    xs: 12,
  };
  // @ts-ignore
  // @ts-ignore
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
            subtitle="Your Portfolio in full view. Check on your positions and claim rewards."
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
          <Grid item {...standardPageSize}>
            <Tabs
              items={tabs}
              value={tabValue}
              onChange={(_e, value) => setTabValue(value)}
            />

            {/* My Assets Tab Panels */}
            {/* Pass in more token ids to show here */}
            <TabPanel value={tabValue} index={0}>
              <MyAssetsTable tokensToList={["pica"]} />
            </TabPanel>

            {/* My Staking Tab Panels */}
            <TabPanel value={tabValue} index={1}>
              <Box px={2}>
                <PageTitle title="Picasso" textAlign="left" fontSize={40} />
              </Box>
              <MyAssetsTable tokensToList={["pica"]} />
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
              <MyStakesTable assets={myStakes.pablo} />
            </TabPanel>

            {/* My Bondings Tab Panels */}
            <TabPanel value={tabValue} index={2}>
              <Box px={2}>
                <PageTitle title="Picasso" textAlign="left" fontSize={40} />
              </Box>
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
