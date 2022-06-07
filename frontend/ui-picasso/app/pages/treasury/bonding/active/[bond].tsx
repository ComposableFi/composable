import { NextPage } from "next";
import { useRouter } from "next/router";
import { Box, Button, Grid, Stack, Typography, useTheme } from "@mui/material";
import { alpha } from "@mui/material/styles";

import { BondBox, Input } from "@/components";
import Default from "@/components/Templates/Default";
import { PageTitle } from "@/components/Molecules";
import PositionDetailsRow from "@/components/Atom/PositionDetailsRow";
import { useAppSelector } from "@/hooks/store";
import {
  bondPrice,
  discount,
  marketPrice,
  soldOut,
  vestingPeriod,
} from "@/stores/defi/stats/dummyData";

const standardPageSize = {
  xs: 12,
};

type BoxData = {
  title: string;
  description: string;
  discountColor?: number;
};

const Bond: NextPage = () => {
  const router = useRouter();
  const theme = useTheme();
  const token = router.query.token as string;
  const toToken = router.query.toToken as string;
  const { bond, claim } = useAppSelector(
    (state) => state.statsTreasury.treasuryBonding
  );

  const bondBoxes: BoxData[] = [
    {
      title: "Bond price",
      description: `$${bondPrice}`,
    },
    {
      title: "Market price",
      description: `$${marketPrice}`,
    },
    {
      title: "Discount",
      description: `${discount}%`,
      discountColor: discount,
    },
    {
      title: "Vesting period",
      description: `${vestingPeriod} days`,
    },
  ];

  return (
    <Default>
      <Box flexGrow={1} sx={{ mx: "auto" }} maxWidth={1032} paddingBottom={16}>
        <Grid container alignItems="center">
          <Grid item {...standardPageSize} mt={theme.spacing(9)}>
            <PageTitle
              title={`${token}-${toToken}`}
              subtitle="Purchase CHAOS at a discount"
              textAlign="center"
            />
          </Grid>
          <Grid item container spacing={3} mt={theme.spacing(9)}>
            {bondBoxes.map(({ title, description, discountColor }) => (
              <Grid item key={title} xs={3}>
                <BondBox
                  title={title}
                  description={description}
                  discountColor={discountColor}
                />
              </Grid>
            ))}
          </Grid>

          <Grid
            item
            {...standardPageSize}
            mt="2rem"
            container
            spacing={theme.spacing(3)}
          >
            <Grid item xs={6}>
              <Box
                sx={{
                  display: "grid",
                  alignItems: "center",
                  padding: "3rem",
                  backgroundColor: alpha(theme.palette.common.white, 0.02),
                  borderRadius: "0.75rem",
                }}
              >
                <Typography
                  variant="h5"
                  color="text.common.white"
                  textAlign="left"
                  mb="2rem"
                >
                  Bond
                </Typography>

                <Input value="" disabled />

                <Button
                  sx={{
                    mt: theme.spacing(4),
                  }}
                  variant="contained"
                  fullWidth
                  onClick={() => {}}
                  disabled={soldOut ? true : false}
                >
                  {soldOut ? "Sold Out" : "Bond"}
                </Button>

                <Stack mt={theme.spacing(4)}>
                  {bond.map(({ label, description }) => (
                    <PositionDetailsRow
                      key={label}
                      label={label}
                      description={description}
                      soldOut={soldOut}
                    />
                  ))}
                </Stack>
              </Box>
            </Grid>

            <Grid item xs={6}>
              <Box
                sx={{
                  display: "grid",
                  alignItems: "center",
                  padding: "3rem",
                  backgroundColor: alpha(theme.palette.common.white, 0.02),
                  borderRadius: "0.75rem",
                }}
              >
                <Typography
                  variant="h5"
                  color="text.common.white"
                  textAlign="left"
                  mb="2rem"
                >
                  Claim
                </Typography>

                <Input value="" disabled />

                <Button
                  sx={{
                    mt: theme.spacing(4),
                  }}
                  variant="contained"
                  fullWidth
                  onClick={() => {}}
                >
                  Claim
                </Button>

                <Stack mt={theme.spacing(4)}>
                  {claim.map(({ label, description }) => (
                    <PositionDetailsRow
                      key={label}
                      label={label}
                      description={description}
                    />
                  ))}
                </Stack>
              </Box>
            </Grid>
          </Grid>
        </Grid>
      </Box>
    </Default>
  );
};

export default Bond;
