import { Box, Typography, useTheme } from "@mui/material";
import { FC } from "react";

export const Message: FC<{ title?: string; description?: string }> = ({
  title,
  description,
}) => {
  const theme = useTheme();
  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        alignItems: "start",
        justifyContent: "flex-start",
        gap: "0.5rem",
      }}
    >
      <Typography variant="body2" color={theme.palette.common.white}>
        {title}
      </Typography>
      <Typography
        variant="body2"
        sx={{ lineHeight: 0.5 }}
        color={theme.palette.common.white}
      >
        {description}
      </Typography>
    </Box>
  );
};
