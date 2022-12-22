import { MessageAction } from "@/components/Molecules/Snackbar/Action";
import { Message } from "@/components/Molecules/Snackbar/Message";
import { themeOverride } from "@/components/Molecules/Snackbar/themeOverride";
import { CheckCircle, Error } from "@mui/icons-material";
import { Alert, AlertColor, Box } from "@mui/material";
import { ThemeProvider, useTheme } from "@mui/material/styles";
import { CustomContentProps, useSnackbar } from "notistack";
import { keyframes } from "@emotion/react";
import {
  forwardRef,
  memo,
  useCallback,
  useMemo,
  useRef,
  useState,
} from "react";

const progress = keyframes([
  {
    from: {
      width: "90%",
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
  timeout: number;
};

const ThemeResponsiveSnackbarComp = forwardRef<
  HTMLDivElement,
  NotificationOverrideProps
>(
  (
    { variant, timeout, id, isClosable, message, description, url },
    forwardedRef
  ) => {
    const { closeSnackbar } = useSnackbar();
    const linearProgressRef = useRef(null);
    const defaultTheme = useTheme();
    const theme = useMemo(
      () => themeOverride(defaultTheme, variant),
      [defaultTheme, variant]
    );
    const [paused, setPaused] = useState<boolean>(false);
    const [width, setWidth] = useState<string | number>("90%");

    const handleSnackbarClose = useCallback(() => {
      if (isClosable && !paused && timeout !== 0) {
        setWidth(0);
        closeSnackbar(id);
      }
    }, [closeSnackbar, id]); // eslint-disable-line react-hooks/exhaustive-deps

    return (
      <ThemeProvider theme={theme}>
        <Box
          ref={forwardedRef}
          onMouseEnter={() => setPaused(true)}
          onMouseLeave={() => setPaused(false)}
          sx={{
            borderRadius: 1,
            overflow: "hidden",
            display: "flex",
            flexDirection: "column",
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
                onClose={handleSnackbarClose}
                url={url}
              />
            }
            sx={{
              padding: 2,
            }}
          >
            <Box
              sx={{
                display: "flex",
                width: "100%",
                justifyContent: "space-between",
                alignItems: "center",
                height: theme.spacing(8),
              }}
            >
              <Message title={message} description={description} />
            </Box>
          </Alert>
          <Box
            style={{
              width,
            }}
            sx={{
              alignSelf: "center",
            }}
          >
            <Box
              ref={linearProgressRef}
              sx={{
                height: "0.25rem",
                backgroundColor: theme.palette[variant].main,
                borderBottomRightRadius: "1rem",
                borderBottomLeftRadius: "1rem",
                animation: `${progress} ${timeout}ms linear`,
                animationPlayState: paused ? "paused" : "running",
              }}
              onAnimationEnd={handleSnackbarClose}
            />
          </Box>
        </Box>
      </ThemeProvider>
    );
  }
);

ThemeResponsiveSnackbarComp.displayName = "ThemeResponsiveSnackbarComp";

export const ThemeResponsiveSnackbar = memo(ThemeResponsiveSnackbarComp);
