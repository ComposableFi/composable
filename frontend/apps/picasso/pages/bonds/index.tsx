import { AllBondsTable } from "@/components/Molecules/AllBondsTable";
import { UpcomingFeature } from "@/components/Molecules/UpcomingFeature";
import { BondPageHeading } from "@/components/Organisms/Bond/BondPageHeading";
import Default from "@/components/Templates/Default";
import { alpha, Box, Grid, Typography, useTheme } from "@mui/material";
import BigNumber from "bignumber.js";
import type { NextPage } from "next";

const standardPageSize = {
  xs: 12,
};

const Bonds: NextPage = () => {
  const theme = useTheme();

  return (
    <Default>
      <Box flexGrow={1} sx={{ mx: "auto" }} maxWidth={1032} paddingBottom={16}>
        <Grid container spacing={4}>
          <Grid item {...standardPageSize} mt={theme.spacing(9)}>
            <BondPageHeading />
          </Grid>
          <Grid item {...standardPageSize}>
            <UpcomingFeature>
              <Box
                padding={4}
                borderRadius={1}
                bgcolor={alpha(theme.palette.common.white, 0.02)}
              >
                <Typography mb={2}>All Bonds</Typography>
                <AllBondsTable
                  bonds={
                    [
                      {
                        beneficiary: "" as any,
                        asset: {
                          id: "PICA",
                        },
                        bondPrice: new BigNumber(0),
                        nbOfBonds: 1,
                        maturity: "Infinite",
                        reward: {
                          asset: {
                            id: "pica",
                          },
                          amount: new BigNumber(0),
                          maturity: new BigNumber(1),
                          assetId: "1",
                        },
                        rewardPrice: new BigNumber(0),
                        price: new BigNumber(0),
                        bondOfferId: "1",
                      },
                    ] as any[]
                  }
                  onRowClick={() => {}}
                />
              </Box>
            </UpcomingFeature>
          </Grid>
        </Grid>
      </Box>
    </Default>
  );
};

export default Bonds;
