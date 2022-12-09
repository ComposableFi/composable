import { Box, BoxProps, Typography, useTheme } from "@mui/material";

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
      <Typography variant="body2" marginLeft={theme.spacing(1)}>
        {label}
      </Typography>
    </Box>
  );
};
