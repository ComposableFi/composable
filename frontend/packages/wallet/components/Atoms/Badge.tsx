import { Box, BoxProps, Typography, useTheme } from "@mui/material";
import React from "react";
export type BadgeProps = {
  color: string;
  background: string;
  icon: JSX.Element;
  label: string;
} & BoxProps;

export const Badge = ({
  color,
  background,
  icon,
  label,
  ...props
}: BadgeProps) => {
  const theme = useTheme();
  return (
    <Box
      sx={{
        display: "inline-flex",
        justifyContent: "center",
        alignItems: "center",
        height: "2.124rem",
        color: color,
        background: background,
        borderRadius: "12px",
        px: 1,
      }}
      {...props}
    >
      {icon}
      <Typography variant="inputLabel" marginLeft={theme.spacing(1)}>
        {label}
      </Typography>
    </Box>
  );
};
