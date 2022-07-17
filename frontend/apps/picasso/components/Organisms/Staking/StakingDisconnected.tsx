import { Box, Grid, Stack, Theme, Typography } from "@mui/material";
import { FeaturedBox } from "@/components";
import Image from "next/image";
import { FC } from "react";

interface StakingDisconnectedParams {
  gridSize: { xs: number };
  theme: Theme;
}

export const StakingDisconnected: FC<StakingDisconnectedParams> = ({
  gridSize,
  theme,
}) => (
  <Grid container spacing={8} marginTop={6}>
    <Grid item {...gridSize}>
      <FeaturedBox
        sx={{
          padding: theme.spacing(6, 0),
        }}
        title={"Connect wallet"}
        textBelow="To start staking, wallet needs to be connected."
      />
    </Grid>
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
            Connect to stake your assets.
          </Typography>
        </Stack>
      </Box>
    </Grid>
  </Grid>
);
