import { Box, Grid, Typography, Card, BoxProps } from "@mui/material";
import { YourBondTable } from "@/components/Organisms/YourBondTable";
import { AllBondTable } from "@/components/Organisms/AllBondTable";
import { BoxWrapper } from "../BoxWrapper";

const standardPageSize = {
  xs: 12,
};

export const Bonds: React.FC<BoxProps> = ({ ...boxProps }) => {
  return (
    <Box {...boxProps}>
      <BoxWrapper mt={8}>
        <Typography variant="h6" mb={3}>
          Your active bonds
        </Typography>
        <YourBondTable />
      </BoxWrapper>

      <Grid mt={8}>
        <Grid item {...standardPageSize}>
          <Card variant="outlined">
            <Typography variant="h6" mb={2}>
              All bonds
            </Typography>
            <AllBondTable />
          </Card>
        </Grid>
      </Grid>
    </Box>
  );
};
