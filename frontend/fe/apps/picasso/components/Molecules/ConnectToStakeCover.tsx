import * as React from "react";
import Image from "next/image";
import { Box, Typography } from "@mui/material";

export const ConnectToStakeCover: React.FC<{ message: string }> = ({
  message,
}) => {
  return (
    <Box textAlign="center">
      <Image
        src="/static/lemonade.png"
        css={{ mixBlendMode: "luminosity" }}
        width="96"
        height="96"
        alt="No asset image"
      />
      <Typography variant="body2" paddingTop={4} color="text.secondary">
        {message}
      </Typography>
    </Box>
  );
};
