import type { NextPage } from "next";
import Default from "@/components/Templates/Default";
import { useTheme, Grid, Box } from "@mui/material";
import { PageTitle } from "@/components";
import { StatsTreasuryTab } from "@/components/Organisms/StatsTreasuryTab";

const TreasuryDashboard: NextPage = () => {
  const theme = useTheme();

  const standardPageSize = {
    xs: 12,
  };

  return (
    <Default>
      <Box flexGrow={1} sx={{ mx: "auto" }} maxWidth={1032} paddingBottom={16}>
        <Grid container spacing={4}>
          <Grid item {...standardPageSize} mt={theme.spacing(9)}>
            <PageTitle
              title="Treasury"
              subtitle="Bond and Stake your assets to earn additional rewards"
              textAlign="center"
            />
          </Grid>
          <Grid item {...standardPageSize} sx={{ mt: 4 }}>
            <StatsTreasuryTab />
          </Grid>
        </Grid>
      </Box>
    </Default>
  );
};

export default TreasuryDashboard;
