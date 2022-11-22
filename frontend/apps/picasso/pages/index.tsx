import type { NextPage } from "next";
import Default from "@/components/Templates/Default";
import { Box, Grid, useTheme } from "@mui/material";
import {
  ConnectWalletFeaturedBox,
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
import { CrowdloanRewardsSoon } from "@/components/Molecules/CrowdloanRewardSoon";

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
          <CrowdloanRewardsSoon />
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
      </Grid>
    </Default>
  );
};

export default Overview;
