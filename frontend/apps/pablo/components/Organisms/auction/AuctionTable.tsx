import {
  alpha,
  Box,
  Button,
  Grid,
  Tooltip,
  Typography,
  useTheme,
} from "@mui/material";
import { AllAuctionsTable } from "@/components";
import { useState } from "react";
import { useDotSamaContext } from "substrate-react";

const standardPageSize = {
  xs: 12,
};

export const AuctionTable = () => {
  const theme = useTheme();
  const [enabledCreate] = useState<boolean>(false);
  const { extensionStatus } = useDotSamaContext();

  return (
    <Box mb={25}>
      <Grid container mt={4}>
        <Grid item {...standardPageSize}>
          <Box
            padding={4}
            sx={{
              background: theme.palette.gradient.secondary,
              borderRadius: 1,
            }}
            border={`1px solid ${alpha(
              theme.palette.common.white,
              theme.custom.opacity.light
            )}`}
          >
            <Box
              display="flex"
              mb={4}
              justifyContent="space-between"
              alignItems="center"
            >
              <Typography variant="h6">All liquidity</Typography>
              <Box>
                <Tooltip
                  title={extensionStatus !== "connected" ? "Coming soon" : ""}
                  arrow
                >
                  {enabledCreate ? (
                    <Button
                      onClick={() => {}}
                      variant="contained"
                      size="small"
                      disabled
                    >
                      Create auction
                    </Button>
                  ) : (
                    <Box
                      sx={{
                        padding: theme.spacing(1.5, 3),
                        background: alpha(
                          theme.palette.primary.main,
                          theme.custom.opacity.main
                        ),
                        borderRadius: 9999,
                      }}
                    >
                      <Typography variant="button" color="text.secondary">
                        Create auction
                      </Typography>
                    </Box>
                  )}
                </Tooltip>
              </Box>
            </Box>
            <AllAuctionsTable />
          </Box>
        </Grid>
      </Grid>
    </Box>
  );
};
