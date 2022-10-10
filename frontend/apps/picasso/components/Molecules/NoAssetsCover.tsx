import * as React from "react";
import Image from "next/image";
import { Box, Typography } from "@mui/material";
import { getImageURL } from "@/utils/nextImageUrl";

export const NoAssetsCover: React.FC = () => {
  return (
    <Box textAlign="center">
      <Typography variant="body1" paddingBottom={4}>
        Assets
      </Typography>
      <Image
        alt="lemonade"
        src={getImageURL("/static/lemonade.png")}
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
