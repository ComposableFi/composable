import { Box } from "@mui/material";
import { FC } from "react";

export const UpcomingFeature: FC = ({ children }) => {
  return (
    <Box
      sx={{
        filter: "blur(2.5px)",
      }}
    >
      {children}
    </Box>
  );
};
