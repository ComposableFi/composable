import type { NextPage } from "next";
import { Container, Box, Grid, Typography, useTheme } from "@mui/material";
import Default from "@/components/Templates/Default";
import { useAppSelector } from "@/hooks/store";
import { PageTitle } from "@/components/Organisms/bonds/PageTitle";
import { BuyButtons } from "@/components/Organisms/bonds/BuyButtons";
import { SupplySummary } from "../../../components/Organisms/bonds/SupplySummary";
import { DepositForm } from "@/components/Organisms/bonds/DepositForm";
import { ClaimForm } from "@/components/Organisms/bonds/ClaimForm";
import { useDotSamaContext } from "substrate-react";
import { Link } from "@/components";
import { useDispatch } from "react-redux";
import { useEffect } from "react";
import { useRouter } from "next/router";
import { useSnackbar } from "notistack";
import { useSupplySummary } from "../../../store/hooks/useSupplySummary";
import { useDepositSummary } from "../../../store/hooks/useDepositSummary";

const standardPageSize = {
  xs: 12,
};

const twoColumnPageSize = {
  xs: 12,
  md: 6,
};

const SelectBond: NextPage = () => {
  const theme = useTheme();
  const drawerWidth = theme.custom.drawerWidth.desktop;
  const router = useRouter();
  const dispatch = useDispatch();
  const { enqueueSnackbar } = useSnackbar();
  const { extensionStatus } = useDotSamaContext();
  const bond = useAppSelector((state) => state.bonds.selectedBond);
  const offerId = Number(router.query.offerId);

  const supplySummary = useSupplySummary({ offerId });
  const depositSummary = useDepositSummary({ offerId });

  const claimable = !bond.claimable_amount.eq(0) || !bond.pending_amount.eq(0);

  const message = useAppSelector((state) => state.ui.message);

  useEffect(() => {
    if (extensionStatus !== "connected") {
      router.push("/bond");
    }
  }, [extensionStatus]);

  useEffect(() => {
    if (message.text) {
      enqueueSnackbar(message.text, {
        description: message.text,
        variant: message.severity,
        isClosable: true,
        url: message.link,
      });
    }
  }, [enqueueSnackbar, message]);

  if (supplySummary === "no-summary" || depositSummary === "no-summary") {
    return null;
  }

  const breadcrumbs = [
    <Link key="pool" underline="none" color="primary" href="/bond">
      <Typography key="addliquidity" variant="body1">
        Bonds
      </Typography>
    </Link>,
    <Typography key="addliquidity" variant="body1" color="text.primary">
      Bond select
    </Typography>,
  ];

  return (
    <Default breadcrumbs={breadcrumbs}>
      <Container maxWidth="lg">
        <Box display="flex" flexDirection="column" alignItems="center">
          <PageTitle
            tokenId1={
              "base" in supplySummary.principalAsset
                ? supplySummary.principalAsset.base.id
                : supplySummary.principalAsset.id
            }
            tokenId2={
              "base" in supplySummary.principalAsset
                ? supplySummary.principalAsset.quote.id
                : supplySummary.principalAsset.id
            }
            rewardAsset={supplySummary.rewardAsset}
          />
        </Box>

        <BuyButtons mt={8} bond={bond} />

        <SupplySummary mt={8} supplySummary={supplySummary} />
        <Box position="relative" mt={8} mb={25}>
          <Grid container columnSpacing={4}>
            <Grid item {...(claimable ? twoColumnPageSize : standardPageSize)}>
              <DepositForm
                bond={bond}
                supplySummary={supplySummary}
                depositSummary={depositSummary}
              />
            </Grid>
            {claimable && (
              <Grid item {...twoColumnPageSize}>
                <ClaimForm bond={bond} />
              </Grid>
            )}
          </Grid>
        </Box>
      </Container>
    </Default>
  );
};

export default SelectBond;
