import { forwardRef, memo, useCallback, useRef, useState } from "react";
import { CheckCircleOutline, ErrorOutline } from "@mui/icons-material";
import { Alert, AlertColor, Box } from "@mui/material";
import { useTheme } from "@mui/material/styles";
import { CustomContentProps, useSnackbar } from "notistack";
import { keyframes } from "@emotion/react";

import { MessageAction } from "@/components/Molecules/Snackbar/Action";
import { Message } from "@/components/Molecules/Snackbar/Message";

const SNACKBAR_TIMEOUT_DURATION = 30000;

const progress = keyframes([
  {
    from: {
      width: "100%",
    },
    to: {
      width: "0",
    },
  },
]);

type NotificationOverrideProps = CustomContentProps & {
  description?: string;
  url?: string;
  isClosable?: boolean;
};

const ThemeResponsiveSnackbarComp = forwardRef<
  HTMLDivElement,
  NotificationOverrideProps
>(({ variant, id, isClosable, message, description, url }, forwardedRef) => {
  const { closeSnackbar } = useSnackbar();
  const linearProgressRef = useRef(null);
  const theme = useTheme();
  const [paused, setPaused] = useState<boolean>(false);

  const handleSnackbarClose = useCallback(() => {
    closeSnackbar(id);
  }, [closeSnackbar, id]);

  return (
    <Box
      ref={forwardedRef}
      onMouseEnter={() => setPaused(true)}
      onMouseLeave={() => setPaused(false)}
    >
      <Alert
        variant="filled"
        severity={variant as AlertColor}
        iconMapping={{
          success: <CheckCircleOutline fontSize="inherit" />,
          warning: <ErrorOutline fontSize="inherit" />,
          error: <ErrorOutline fontSize="inherit" />,
          info: <ErrorOutline fontSize="inherit" />,
        }}
        action={
          <MessageAction
            variant={variant as AlertColor}
            isClosable={isClosable}
            onClose={handleSnackbarClose}
            url={url}
          />
        }
        sx={{
          overflow: "hidden",
        }}
      >
        <Box
          sx={{
            display: "flex",
            width: "100%",
            justifyContent: "space-between",
            alignItems: "center",
          }}
        >
          <Message title={message} description={description} />
        </Box>
        <Box
          sx={{
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
            position: "absolute",
            overflow: "hidden",
            borderRadius: "0.75rem",
          }}
        >
          <Box
            ref={linearProgressRef}
            sx={{
              position: "absolute",
              bottom: 0,
              left: 0,
              height: "0.125rem",
              backgroundColor: theme.palette[variant as AlertColor].main,
              animation: `${progress} ${SNACKBAR_TIMEOUT_DURATION}ms linear`,
              animationPlayState: paused ? "paused" : "running",
            }}
            onAnimationEnd={handleSnackbarClose}
          />
        </Box>
      </Alert>
    </Box>
  );
});

ThemeResponsiveSnackbarComp.displayName = "ThemeResponsiveSnackbarComp";

export const ThemeResponsiveSnackbar = memo(ThemeResponsiveSnackbarComp);
