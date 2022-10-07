import type { NextPage } from "next";
import { Box, Grid, Link, Typography, useTheme } from "@mui/material";
import { useRouter } from "next/router";
import { useEffect } from "react";
import { PageTitle, FeaturedBox, SS8WalletHelper } from "@/components";
import { ConnectorType, useBlockchainProvider, useConnector } from "bi-lib";
import { useSelectedAccount } from "@/defi/polkadot/hooks";
import { useDotSamaContext } from "substrate-react";
import { useCrowdloanRewardsEligibility } from "@/stores/defi/polkadot/crowdloanRewards/hooks";
import { DEFAULT_EVM_ID } from "@/defi/polkadot/constants";
import Default from "@/components/Templates/Default";
import Image from "next/image";

const CrowdloanRewards: NextPage = () => {
  const theme = useTheme();
  const router = useRouter();
  const { account } = useBlockchainProvider(DEFAULT_EVM_ID);
  const selectedAccount = useSelectedAccount();

  const breadcrumbs = [
    <Link
      key="Overview"
      underline="none"
      color="primary"
      href="/frontend/fe/apps/picasso/pages"
    >
      Overview
    </Link>,
    <Typography key="claims" color="text.secondary">
      Crowdloan Rewards
    </Typography>,
  ];
  const standardPageSize = {
    xs: 12,
  };

  const { extensionStatus } = useDotSamaContext();
  const { isActive } = useConnector(ConnectorType.MetaMask);

  const { isEthAccountEligible, isPicassoAccountEligible } =
    useCrowdloanRewardsEligibility(
      account?.toLowerCase(),
      selectedAccount?.address
    );

  useEffect(() => {
    if (isEthAccountEligible || isPicassoAccountEligible) {
      router.push("/crowdloan-rewards/claim");
    }
  }, [
    router,
    account,
    selectedAccount,
    isEthAccountEligible,
    isPicassoAccountEligible,
  ]);

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
                  router.push("/crowdloan-rewards/claim");
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
                  router.push("/crowdloan-rewards/claim");
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
