import { Stack, Typography, useTheme } from "@mui/material";
import { FC } from "react";

export const Message: FC<{ title?: string; description?: string }> = ({
  title,
  description,
}) => {
  const theme = useTheme();
  return (
    <Stack gap={1}>
      <Typography
        variant="body2"
        color={theme.palette.common.white}
        sx={{
          lineHeight: 1,
        }}
      >
        {title}
      </Typography>
      <Typography
        variant="inputLabel"
        sx={{ lineHeight: 1 }}
        color={theme.palette.common.white}
      >
        {description}
      </Typography>
    </Stack>
  );
};
