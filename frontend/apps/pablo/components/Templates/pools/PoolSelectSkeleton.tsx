import Default from "@/components/Templates/Default";
import { Link, PageTitle } from "@/components";
import { Box, Container, Grid, Skeleton, Typography } from "@mui/material";

const breadcrumbs = [
  <Link key="pool" underline="none" color="primary" href="/pool">
    Pool
  </Link>,
  <Typography key="create-pool" color="text.primary">
    Select
  </Typography>,
];

export const PoolSelectSkeleton = () => {
  return (
    <Default breadcrumbs={breadcrumbs}>
      <Container maxWidth="lg">
        <Box display="flex" flexDirection="column" alignItems="center" mb={8}>
          <PageTitle
            title={"..."}
            subtitle="Earn tokens while adding liquidity."
          />
        </Box>
        <Grid container>
          <Grid item xs={12} md={6}>
            <Skeleton
              variant="rectangular"
              sx={{
                height: "450px",
              }}
            />
          </Grid>
          <Grid item xs={12} md={6}>
            <Skeleton
              variant="rectangular"
              sx={{
                height: "450px",
              }}
            />
          </Grid>
          <Grid item xs={12}>
            <Skeleton
              variant="rectangular"
              sx={{
                height: "450px",
              }}
            />
          </Grid>
        </Grid>
      </Container>
    </Default>
  );
};
