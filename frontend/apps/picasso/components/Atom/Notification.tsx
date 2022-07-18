import { FC } from "react";
import Alert, { AlertColor } from "@mui/material/Alert";
import Typography from "@mui/material/Typography";
import { useTheme } from "@mui/material/styles";
import useMediaQuery from "@mui/material/useMediaQuery";

export type NotificationProps = {
  severity: AlertColor;
  alertText: string;
};

export const Notification: FC<NotificationProps> = ({
  severity,
  alertText,
}) => {
  const theme = useTheme();
  const isDesktop = useMediaQuery(theme.breakpoints.up("md"));

  return (
    <Alert
      sx={{
        height: isDesktop ? 56 : 48,
      }}
      variant="filled"
      severity={severity}
    >
      <Typography
        sx={{
          color: theme.palette.common.white,
          fontFamily: "Be Vietnam Pro",
        }}
        variant="body2"
      >
        {alertText}
      </Typography>
    </Alert>
  );
};
