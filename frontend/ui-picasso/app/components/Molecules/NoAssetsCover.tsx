import * as React from "react";
import Image from "next/image";
import { Box, Paper, Typography } from "@mui/material";

export const NoAssetsCover: React.FC = () => {
  return (
    <Box textAlign="center">
      <Typography variant="body1" paddingBottom={4}>
        Assets
      </Typography>
      <Image
        alt="lemonade"
        src="/static/lemonade.png"
        css={{ mixBlendMode: "luminosity" }}
        width="96"
        height="96"
      />
      <Typography variant="body2" paddingTop={4} color="text.secondary">
        No assets
      </Typography>
    </Box>
  );
};
