import { useState } from "react";
import { NextPage } from "next";
import { useRouter } from "next/router";
import { Box, Button, Grid, Stack, Typography, useTheme } from "@mui/material";
import WarningAmberRoundedIcon from "@mui/icons-material/WarningAmberRounded";

import { BondBox, Input } from "@/components";
import Default from "@/components/Templates/Default";
import { Modal, PageTitle } from "@/components/Molecules";
import PositionDetails from "@/components/Atom/PositionDetails";
import PositionDetailsRow from "@/components/Atom/PositionDetailsRow";
import {
  balance,
  bondPrice,
  discount,
  marketPrice,
  maxToBuy,
  reward,
  roi,
  vestingPeriod,
} from "@/stores/defi/stats/dummyData";

const standardPageSize = {
  xs: 12,
};

type PositionIndex = 0 | 1 | 2 | 3 | 4;

type PositionData = {
  label: string;
  description: string;
};

type PositionItem = {
  [key in PositionIndex]: PositionData;
};

type BoxPosition = 0 | 1 | 2 | 3;

type BoxData = {
  title: string;
  description: string;
  discountColor?: number;
};

type BoxItem = {
  [key in BoxPosition]: BoxData;
};

const Bond: NextPage = () => {
  const router = useRouter();
  const theme = useTheme();
  const [open, setOpen] = useState<boolean>(false);
  const [open2nd, setOpen2nd] = useState<boolean>(false);
  const token = router.query.token as string;
  const toToken = router.query.toToken as string;

  const bondBoxes: BoxItem = {
    0: {
      title: "Bond price",
      description: `$${bondPrice}`,
    },
    1: {
      title: "Market price",
      description: `$${marketPrice}`,
    },
    2: {
      title: "Discount",
      description: `${discount}%`,
      discountColor: discount,
    },
    3: {
      title: "Vesting period",
      description: `${vestingPeriod} days`,
    },
  };

  const positionRows: PositionItem = {
    0: {
      label: "Your balance",
      description: `${balance} LP`,
    },
    1: {
      label: "You will get",
      description: `${reward} CHAOS`,
    },
    2: {
      label: "Max you can buy",
      description: `${maxToBuy} CHAOS`,
    },
    3: {
      label: "Vesting term",
      description: `${vestingPeriod} days`,
    },
    4: {
      label: "ROI",
      description: `${roi}%`,
    },
  };

  const confirmationRows = [
    {
      label: "Bonding",
      description: `${balance} LP`,
    },
    {
      label: "You will get",
      description: `${reward} CHAOS`,
    },
    {
      label: "Bond price",
      description: `$${bondPrice}`,
    },
    {
      label: "Market price",
      description: `$${marketPrice}`,
    },
    {
      label: "Discount",
      description: `${discount}%`,
      discountColor: discount,
    },
  ];

  const handleApprove = () => {
    setOpen(true);
    // Approve logic here
  };

  const handlePurchase = () => {
    if (discount < 0) setOpen2nd(true);
    // Purchase logic here
  };

  const handleWait = () => {
    setOpen(false);
    setOpen2nd(false);
  };

  const handleBurnMoney = () => {
    setOpen(false);
    setOpen2nd(false);
    // Purchase with negative discount here
  };

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
            {Object.values(bondBoxes).map(
              ({ title, description, discountColor }) => (
                <Grid item key={title} xs={3}>
                  <BondBox
                    title={title}
                    description={description}
                    discountColor={discountColor}
                  />
                </Grid>
              )
            )}
          </Grid>
          <Grid item {...standardPageSize} mt="4.5rem">
            <Typography
              variant="h5"
              color="text.common.white"
              textAlign="center"
              mb="3.813rem"
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
              onClick={handleApprove}
            >
              Approve
            </Button>

            {/** First confirmation */}
            <Modal open={open} onClose={() => setOpen(false)} dismissible>
              <Typography textAlign="center" variant="h6">
                Purchase Bond
              </Typography>
              {discount < 0 && (
                <Typography
                  textAlign="center"
                  variant="subtitle2"
                  color="text.secondary"
                  mt={theme.spacing(2)}
                >
                  Are you sure you want to bond for a negative discount? <br />
                  You will lose money if you do this...
                </Typography>
              )}
              <Stack mt="4rem">
                {confirmationRows.map(
                  ({ label, description, discountColor }) => (
                    <PositionDetailsRow
                      key={label}
                      label={label}
                      description={description}
                      descriptionColor={discountColor}
                    />
                  )
                )}
              </Stack>
              <Button
                sx={{
                  mt: theme.spacing(4),
                }}
                variant="contained"
                fullWidth
                onClick={handlePurchase}
              >
                Purchase Bond
              </Button>
              <Button
                sx={{
                  mt: theme.spacing(4),
                }}
                variant="text"
                fullWidth
                onClick={() => setOpen(false)}
              >
                Cancel Bond
              </Button>
            </Modal>

            {/** Second confirmation */}
            <Modal
              open={open2nd}
              onClose={() => {
                setOpen(false);
                setOpen2nd(false);
              }}
              dismissible
            >
              <Box textAlign="center" mb={theme.spacing(6)}>
                <WarningAmberRoundedIcon
                  sx={{
                    color: "text.secondary",
                    width: 80,
                    height: 80,
                  }}
                />
              </Box>
              <Typography textAlign="center" variant="h6">
                Warning
              </Typography>
              <Typography
                textAlign="center"
                variant="subtitle2"
                color="text.secondary"
                mt={theme.spacing(2)}
              >
                This bond is currently at a negative discount. <br />
                Please consider waiting until bond returns to positive discount.
              </Typography>
              <Button
                sx={{
                  mt: theme.spacing(8),
                }}
                variant="contained"
                fullWidth
                onClick={handleWait}
              >
                {"Ok, I'll wait"}
              </Button>
              <Button
                sx={{
                  mt: theme.spacing(4),
                }}
                variant="text"
                fullWidth
                onClick={handleBurnMoney}
              >
                I want to burn money
              </Button>
            </Modal>
          </Grid>
          <Grid item {...standardPageSize} mt={theme.spacing(9)}>
            <PositionDetails>
              {Object.values(positionRows).map(({ label, description }) => (
                <PositionDetailsRow
                  key={label}
                  label={label}
                  description={description}
                />
              ))}
            </PositionDetails>
          </Grid>
        </Grid>
      </Box>
    </Default>
  );
};

export default Bond;
