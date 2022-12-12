import { AllBondTable } from "@/components/Organisms/AllBondTable";
import { YourBondTable } from "@/components/Organisms/YourBondTable";
import { Box, BoxProps, Grid, Typography } from "@mui/material";
import { BoxWrapper } from "../BoxWrapper";

const standardPageSize = {
  xs: 12,
};

export const Bonds: React.FC<BoxProps> = ({ ...boxProps }) => {
  return (
    <Box {...boxProps}>
      <Grid mt={8}>
        <Grid item {...standardPageSize}>
          <BoxWrapper>
            <Typography variant="h6">
              All bonds
            </Typography>
            <AllBondTable />
          </BoxWrapper>
        </Grid>
      </Grid>
    </Box>
  );
};
