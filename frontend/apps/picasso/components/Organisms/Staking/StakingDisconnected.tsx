import { Box, Grid, Stack, Typography } from "@mui/material";
import Image from "next/image";
import { FC } from "react";
import { StakingHighlights } from "@/components/Organisms/Staking/StakingHighlights";

interface StakingDisconnectedParams {
  gridSize: { xs: number };
}

export const StakingDisconnected: FC<StakingDisconnectedParams> = ({
  gridSize,
}) => (
  <>
    <StakingHighlights />
    <Grid container marginTop={6} spacing={9}>
      <Grid item {...gridSize}>
        <Box>
          <Stack>
            <Image
              style={{ mixBlendMode: "luminosity" }}
              src="/static/Rocket.svg"
              width={200}
              height={200}
              alt="rocket orbiting the moon"
            />
            <Typography variant="h6" textAlign="center" color="text.secondary">
              Connect your wallet and start earning.
            </Typography>
          </Stack>
        </Box>
      </Grid>
    </Grid>
  </>
);
