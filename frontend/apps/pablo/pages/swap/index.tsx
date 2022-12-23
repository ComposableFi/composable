import type { NextPage } from "next";
import { Box, Container, Grid } from "@mui/material";
import AccountSettings from "@/components/Organisms/TransactionSettings/AccountSettings";
import { PageTitle } from "@/components";
import SwapForm from "@/components/Organisms/swap/SwapForm";
import { PoolLayout } from "@/components/Templates/pools/PoolLayout";

const standardPageSize = {
  xs: 12,
};

const Swap: NextPage = () => {
  return (
    <PoolLayout>
      <Container maxWidth="lg">
        <Box mb={25}>
          <Box textAlign="center">
            <PageTitle title="Swap" />
          </Box>
          <Grid mt={4} container spacing={4}>
            <Grid item {...standardPageSize}>
              <SwapForm />
            </Grid>
          </Grid>
          <AccountSettings />
        </Box>
      </Container>
    </PoolLayout>
  );
};

export default Swap;
