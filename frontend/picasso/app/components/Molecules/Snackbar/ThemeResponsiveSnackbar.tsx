import { CustomContentProps, useSnackbar } from "notistack";
import { forwardRef, memo, useEffect, useMemo, useState } from "react";
import {
  Box,
  Alert,
  AlertColor,
  ThemeProvider,
  alpha,
  ThemeOptions,
} from "@mui/material";
import { CheckCircle, Error } from "@mui/icons-material";
import { MessageAction } from "./Action";
import { Message } from "./Message";
import { ProgressBar } from "./ProgressBar";
import { createTheme, useTheme } from "@mui/material/styles";
import Timer from "tiny-timer";
import { duration } from "@/utils/snackbar";
import { brandPalette } from "@/styles/theme";

interface NotificationOverrideProps extends CustomContentProps {
  description?: string;
  url?: string;
  isClosable?: boolean;
}

const ThemeResponsiveSnackbarComp = forwardRef<
  HTMLDivElement,
  NotificationOverrideProps
>((props, forwardedRef) => {
  const { closeSnackbar } = useSnackbar();
  const defaultTheme = useTheme();
  const [timeoutTimer, _] = useState(new Timer({ interval: 60 }));
  const [progress, setProgress] = useState<number>(100);
  const {
    message,
    variant,
    description,
    isClosable,
    url,
    id,
    autoHideDuration,
    persist,
  } = props;
  const timeOut = autoHideDuration || persist ? duration : 0;
  const theme = useMemo(
    () =>
      createTheme(defaultTheme, {
        palette: {
          info: {
            main: "#0286FF",
          },
          success: {
            main: "#009B6D",
          },
          error: {
            main: "#E10036",
          },
          warning: {
            main: "#C59A04",
          },
        },
        components: {
          MuiAlert: {
            styleOverrides: {
              action: {
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                padding: 0,
              },
              message: {
                display: "flex",
                flexDirection: "column",
                alignItems: "start",
                justifyContent: "flex-start",
                color: defaultTheme.palette.common.white,
              },
              filled: {
                minWidth: "500px",
                backdropFilter: "blur(32px)",
              },
              filledInfo: {
                backgroundColor: alpha(
                  defaultTheme.palette[variant as AlertColor].main,
                  0.1
                ),
                color: defaultTheme.palette.common.white,
              },
              filledSuccess: {
                backgroundColor: alpha(
                  defaultTheme.palette[variant as AlertColor].main,
                  0.1
                ),
                color: defaultTheme.palette.common.white,
              },
              filledError: {
                backgroundColor: alpha(
                  defaultTheme.palette[variant as AlertColor].main,
                  0.1
                ),
                color: defaultTheme.palette.common.white,
              },
              filledWarning: {
                backgroundColor: alpha(
                  defaultTheme.palette[variant as AlertColor].main,
                  0.1
                ),
                color: defaultTheme.palette.common.white,
              },
            },
          },
          MuiLinearProgress: {
            styleOverrides: {
              root: {
                marginTop: "-4px",
                "&::before": {
                  background: "initial",
                  borderTop: `4px solid ${
                    brandPalette[variant as AlertColor].light
                  }`,
                  position: "absolute",
                  top: 0,
                  left: 0,
                  width: "100%",
                  content: '""',
                },
              },
            },
          },
        },
      }) as any,
    []
  );
  const handleClose = () => {
    timeoutTimer.stop();
    closeSnackbar(id);
  };
  const getProgress = (ms: number) => {
    if (timeOut) {
      return Math.round((ms / timeOut) * 100);
    }
    return progress;
  };

  useEffect(() => {
    timeoutTimer.on("tick", (ms: number) => setProgress(getProgress(ms)));
    timeoutTimer.on("done", () => {
      setTimeout(() => {
        closeSnackbar(id);
      }, theme.transitions.duration);
    });
    if (timeOut) {
      timeoutTimer.start(timeOut);
    }
  }, []);

  return (
    <ThemeProvider theme={theme}>
      <Box
        ref={forwardedRef}
        onMouseEnter={() => {
          if (timeOut) {
            timeoutTimer.pause();
          }
        }}
        onMouseLeave={() => {
          if (timeOut) {
            timeoutTimer.resume();
          }
        }}
        sx={{
          borderRadius: `${theme.shape.borderRadius}px`,
          overflow: "hidden",
        }}
      >
        <Alert
          variant="filled"
          severity={variant as AlertColor}
          iconMapping={{
            success: <CheckCircle fontSize="inherit" />,
            warning: <Error fontSize="inherit" />,
            error: <Error fontSize="inherit" />,
            info: <Error fontSize="inherit" />,
          }}
          action={
            <MessageAction
              variant={variant as AlertColor}
              isClosable={isClosable}
              onClose={handleClose}
              url={url}
            />
          }
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
        </Alert>
        {timeOut && (
          <ProgressBar progress={progress} color={variant as AlertColor} />
        )}
      </Box>
    </ThemeProvider>
  );
});

ThemeResponsiveSnackbarComp.displayName = "ThemeResponsiveSnackbarComp";

export const ThemeResponsiveSnackbar = memo(ThemeResponsiveSnackbarComp);
