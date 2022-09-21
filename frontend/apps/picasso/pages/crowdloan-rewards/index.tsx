import type { NextPage } from "next";
import Default from "@/components/Templates/Default";
import { Box, Grid, Link, Typography, useTheme } from "@mui/material";
import { useRouter } from "next/router";
import Image from "next/image";
import { useContext, useEffect } from "react";
import { useStore } from "@/stores/root";
import { ParachainContext } from "@/defi/polkadot/context/ParachainContext";
import { PageTitle, FeaturedBox, SS8WalletHelper } from "@/components";
import { ConnectorType, useConnector } from "bi-lib";

const CrowdloanRewards: NextPage = () => {
  const theme = useTheme();
  const router = useRouter();
  const userAssociation = useStore(
    ({ crowdloanRewards }) => crowdloanRewards.associatedWith
  );

  const breadcrumbs = [
    <Link key="Overview" underline="none" color="primary" href="/frontend/fe/apps/picasso/pages">
      Overview
    </Link>,
    <Typography key="claims" color="text.secondary">
      Crowdloan Rewards
    </Typography>,
  ];
  const standardPageSize = {
    xs: 12,
  };

  const { extensionStatus } = useContext(ParachainContext);
  const { isActive } = useConnector(ConnectorType.MetaMask);

  useEffect(() => {
    if (userAssociation) {
      userAssociation === "ethereum"
        ? router.push("crowdloan-rewards/stablecoin")
        : router.push("crowdloan-rewards/ksm");
    }
    // Only to be called on page load therefore we can omit dependencies.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <Default breadcrumbs={breadcrumbs}>
      <Grid
        container
        sx={{ mx: "auto" }}
        maxWidth={1032}
        rowSpacing={9}
        columns={10}
        direction="column"
        justifyContent="center"
        pb={9}
      >
        <Grid item {...standardPageSize} mt={theme.spacing(9)}>
          <PageTitle
            title="Crowdloan Rewards"
            textAlign="center"
            subtitle="Claim your PICA rewards for both KSM and stablecoin contributions."
          />
        </Grid>
        <Grid item {...standardPageSize}>
          <Box sx={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 3 }}>
            <FeaturedBox
              sx={{ padding: theme.spacing(6, 8) }}
              title="KSM Contribution"
              textAbove={
                <Box
                  sx={{
                    mb: theme.spacing(4),
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                  }}
                >
                  <Image
                    alt="polkadot"
                    src="/tokens/dotsama-kusama.svg"
                    width="64"
                    height="64"
                  />
                </Box>
              }
              textBelow="To see your portfolio, wallet needs to be connected."
              ButtonProps={{
                label: "Claim with Polkadot.js",
                variant: "contained",
                fullWidth: true,
                disabled: extensionStatus !== "connected",
                onClick: () => {
                  router.push("/crowdloan-rewards/ksm");
                },
              }}
            />
            <FeaturedBox
              title="Stablecoin Contribution"
              textBelow="To see your portfolio, wallet needs to be connected."
              textAbove={
                <Box
                  sx={{
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    mb: theme.spacing(4),
                  }}
                >
                  <Image
                    alt="stablecoin"
                    src="/tokens/usd-coin-usdc.svg"
                    width="64"
                    height="64"
                  />
                  <Box sx={{ width: 64, height: 64, marginLeft: "-8px" }}>
                    <Image
                      alt="dai"
                      src="/tokens/dai.svg"
                      width="64"
                      height="64"
                    />
                  </Box>
                  <Box sx={{ width: 64, height: 64, marginLeft: "-8px" }}>
                    <Image
                      alt="tether"
                      src="/tokens/tether.svg"
                      width="64"
                      height="64"
                    />
                  </Box>
                </Box>
              }
              sx={{ paddingY: "3rem", paddingX: "4rem" }}
              ButtonProps={{
                label: "Claim with Metamask",
                disabled: !isActive,
                fullWidth: true,
                variant: "contained",
                onClick: () => {
                  router.push("/crowdloan-rewards/stablecoin");
                },
              }}
            />
          </Box>
        </Grid>
        <Grid item {...standardPageSize}>
          <SS8WalletHelper />
        </Grid>
      </Grid>
    </Default>
  );
};

export default CrowdloanRewards;
