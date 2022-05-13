import { Box, AlertTitle, Typography } from "@mui/material";
import { FC } from "react";

export const Message: FC<{ title?: string; description?: string }> = ({
  title,
  description,
}) => {
  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        alignItems: "start",
        justifyContent: "flex-start",
      }}
    >
      <AlertTitle color="white">{title}</AlertTitle>
      <Typography variant="body2" color="white">
        {description}
      </Typography>
    </Box>
  );
};
