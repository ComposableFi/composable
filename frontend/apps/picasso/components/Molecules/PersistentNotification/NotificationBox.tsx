import { keyframes } from "@emotion/react";
import {
  Box,
  BoxProps,
  Button,
  CircularProgress,
  IconButton,
  Typography,
  useTheme,
} from "@mui/material";
import { FC, useState } from "react";
import {
  CheckCircleRounded,
  CloseTwoTone,
  ErrorRounded,
  OpenInNew,
} from "@mui/icons-material";
import {
  NotificationVariant,
  useNotificationStore,
} from "@/components/Molecules/PersistentNotification/NotificationStore";

const rotate = keyframes`
  0% {
    transform: rotate(0deg);
  } 
  100% {
    transform: rotate(359deg)
  }
`;
type DefaultBoxProps = BoxProps & {
  variant: NotificationVariant;
};

const DefaultBox: FC<DefaultBoxProps> = ({ variant, children, ...props }) => {
  const theme = useTheme();
  const getBackground = () => {
    switch (variant) {
      case "error":
        return `conic-gradient(${theme.palette.error.light}, ${theme.palette.error.main})`;
      case "success":
        return `conic-gradient(${theme.palette.success.light}, ${theme.palette.featured.lemon})`;
      default:
        return `conic-gradient(${theme.palette.secondary.main}, ${theme.palette.primary.main})`;
    }
  };

  return (
    <Box
      sx={{
        position: "absolute",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        top: theme.spacing(4),
        left: theme.spacing(4),
        backgroundColor: theme.palette.background.paper,
        borderRadius: `${theme.shape.borderRadius}px`,
        boxShadow: theme.shadows[1],
        minHeight: "300px",
        [theme.breakpoints.up("md")]: {
          maxWidth: "min(30vw, 400px)",
        },
        [theme.breakpoints.down("sm")]: {
          maxWidth: "100vw",
        },
        zIndex: 9998,
        backgroundOrigin: "border-box",
        overflow: "hidden",
        "&::before": {
          content: "''",
          position: "absolute",
          width: "200%",
          height: "200%",
          background: getBackground(),
          animation: `${rotate} 4s linear infinite normal forwards`,
        },
        "&::after": {
          content: "''",
          position: "absolute",
          inset: theme.spacing(0.5),
          backgroundColor: theme.palette.background.default,
          borderRadius: "8px",
        },
      }}
      {...props}
    >
      {children}
    </Box>
  );
};

export const NotificationBox = () => {
  const notification = useNotificationStore((state) => state.active);
  const archive = useNotificationStore((state) => state.archive);
  const theme = useTheme();
  if (!notification) {
    return notification;
  }
  return (
    <DefaultBox variant={notification.variant}>
      <Box
        sx={{
          zIndex: 9999,
          display: "flex",
          justifyContent: "flex-start",
          alignItems: "center",
          flexDirection: "column",
          height: "100%",
          width: "100%",
          padding: theme.spacing(2),
        }}
        gap={2}
      >
        <IconButton
          sx={{
            position: "absolute",
            top: theme.spacing(1),
            right: theme.spacing(1),
          }}
          onClick={archive}
        >
          <CloseTwoTone />
        </IconButton>
        {notification.variant === "default" && <CircularProgress />}
        {notification.variant === "success" && (
          <CheckCircleRounded fontSize="large" color="success" />
        )}
        {notification.variant === "error" && (
          <ErrorRounded fontSize="large" color="error" />
        )}

        <Typography variant="h5" color="text.primary">
          {notification.title}
        </Typography>
        <Typography>{notification.subtitle}</Typography>
        <Button variant="contained" size="small">
          View on explorer <OpenInNew />
        </Button>
      </Box>
    </DefaultBox>
  );
};
