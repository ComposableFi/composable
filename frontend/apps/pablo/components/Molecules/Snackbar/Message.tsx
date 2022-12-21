import { AlertTitle, Box, Typography, useTheme } from "@mui/material";
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
        height: "fit-content",
      }}
    >
      <AlertTitle sx={{ marginBottom: 0 }} color={theme.palette.common.white}>
        {title}
      </AlertTitle>
      <Typography variant="body2" color={theme.palette.common.white}>
        {description}
      </Typography>
    </Box>
  );
};
