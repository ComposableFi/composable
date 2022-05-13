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
import {
  Chart,
  FeaturedBox,
  PageTitle,
  TabItem,
  TabPanel,
  Tabs,
} from "@/components";
import { ConnectWalletFeaturedBox } from "@/components/Organisms/ConnectWalletFeaturedBox";
import { useAppSelector } from "@/hooks/store";
import { Link } from "@/components";
import { YourBondTable } from "@/components/Organisms/YourBondTable";
import { AllBondTable } from "@/components/Organisms/AllBondTable";
import { useContext, useState } from "react";
import { StakeUnstakeTabPanel } from "@/components/Organisms/StakeUnstakeTabPanel";
import {useDotSamaContext} from "substrate-react";
const tabItems: TabItem[] = [
  {
    label: "Stake",
  },
  {
    label: "Unstake",
  },
];

const standardPageSize = {
  xs: 12,
};

const threeColumnPageSize = {
  xs: 12,
  md: 4,
};

const Staking: NextPage = () => {
  const {extensionStatus} = useDotSamaContext();
  const theme = useTheme();
  const [tab, setTab] = useState(0);

  const handleTabChange = (_: React.SyntheticEvent, newValue: number) => {
    setTab(newValue);
  };

  return (
    <Default>
      <Container maxWidth="lg">
        <Box display="flex" flexDirection="column" alignItems="center" mb={8}>
          <PageTitle title="Stake" subtitle="Stake PABLO for sPABLO" />
        </Box>
        {extensionStatus!=="connected" ? (
          <ConnectWalletFeaturedBox />
        ) : (
          <>
            <Grid container spacing={8}>
              <Grid item {...threeColumnPageSize}>
                <FeaturedBox
                  variant="contained"
                  textAbove="Your deposits"
                  title="$0"
                />
              </Grid>
              <Grid item {...threeColumnPageSize}>
                <FeaturedBox
                  textAbove="APY"
                  title="2,624%"
                  TitleProps={{ color: theme.palette.featured.main }}
                />
              </Grid>
              <Grid item {...threeColumnPageSize}>
                <FeaturedBox textAbove="Total PABLO staked" title="$66.3K" />
              </Grid>
            </Grid>
            <Grid mt={8} {...standardPageSize}>
              <Card>
                <Tabs items={tabItems} value={tab} onChange={handleTabChange} />
                {tab === 0 && (
                  <Box mt={8}>
                    <StakeUnstakeTabPanel activeTab="staking" />
                  </Box>
                )}
                {tab === 1 && (
                  <Box mt={8}>
                    <StakeUnstakeTabPanel activeTab="unstaking" />
                  </Box>
                )}
              </Card>
            </Grid>
          </>
        )}
      </Container>
    </Default>
  );
};

export default Staking;
