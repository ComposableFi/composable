import { FC, Fragment, useState } from "react";
import useMediaQuery from "@mui/material/useMediaQuery";
import Box from "@mui/material/Box";
import { alpha, useTheme } from "@mui/material/styles";
import {
  AlertColor,
  Grid,
  IconButton,
  Snackbar as MuiSnackbar,
  SnackbarOrigin,
  Typography,
} from "@mui/material";
import { Link } from "../Molecules/Link";
import CheckCircleOutlineRoundedIcon from "@mui/icons-material/CheckCircleOutlineRounded";
import ErrorOutlineRoundedIcon from "@mui/icons-material/ErrorOutlineRounded";
import InfoOutlineRoundedIcon from "@mui/icons-material/InfoOutlined";
import WarningAmberRoundedIcon from "@mui/icons-material/WarningAmberRounded";
import OpenInNewRoundedIcon from "@mui/icons-material/OpenInNewRounded";
import CloseRoundedIcon from "@mui/icons-material/CloseRounded";

const SEVERITIES = {
  success: {
    color: "#009B6D",
    background: alpha("#00c68a", 0.1),
    icon: <CheckCircleOutlineRoundedIcon sx={{ color: alpha("#FFF", 0.6) }} />,
  },
  error: {
    color: "#E10036",
    background: alpha("#E10036", 0.1),
    icon: <ErrorOutlineRoundedIcon sx={{ color: alpha("#FFF", 0.6) }} />,
  },
  info: {
    color: "#0286FF",
    background: alpha("#0286FF", 0.1),
    icon: <InfoOutlineRoundedIcon sx={{ color: alpha("#FFF", 0.6) }} />,
  },
  warning: {
    color: "#C59A04",
    background: alpha("#C59A04", 0.1),
    icon: <WarningAmberRoundedIcon sx={{ color: alpha("#FFF", 0.6) }} />,
  },
};

export type SnackbarProps = {
  severity: AlertColor;
  alertText: string;
  href: string;
  show: boolean;
  noAction?: boolean;
};

export interface State extends SnackbarOrigin {
  open: boolean;
}

export const Snackbar: FC<SnackbarProps> = ({
  alertText,
  severity,
  href,
  show,
  noAction = false,
}) => {
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down("sm"));
  const [opened, setOpened] = useState<false | true>(show);

  const handleClose = () => setOpened(false);

  return (
    <MuiSnackbar
      anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
      open={opened}
      onClose={handleClose}
      autoHideDuration={6000}
    >
      <Box
        sx={{
          width: 345,
          height: isMobile ? 56 : 64,
          background: SEVERITIES[severity].background,
          borderRadius: "12px",
          overflow: "hidden",
        }}
      >
        <Box
          sx={{
            height: isMobile ? 56 : 64,
            display: "flex",
            alignItems: "center",
          }}
        >
          <Grid
            container
            alignItems="center"
            justifyContent="space-between"
            margin="0 20px 0 26px"
          >
            <Grid item sx={{ display: "grid" }}>
              {SEVERITIES[severity].icon}
            </Grid>
            <Grid item xs={noAction ? 10 : 6}>
              <Typography
                sx={{
                  color: theme.palette.common.white,
                  fontFamily: "Be Vietnam Pro",
                }}
                variant="body2"
              >
                {alertText}
              </Typography>
            </Grid>
            {!noAction && (
              <Fragment>
                <Grid item>
                  <Link
                    sx={{
                      display: "grid",
                      alignItems: "center",
                    }}
                    href={href}
                    target="_blank"
                    rel="noopener"
                  >
                    <OpenInNewRoundedIcon
                      sx={{ color: SEVERITIES[severity].color }}
                    />
                  </Link>
                </Grid>
                <Grid item>
                  <IconButton onClick={handleClose}>
                    <CloseRoundedIcon
                      sx={{ color: SEVERITIES[severity].color }}
                    />
                  </IconButton>
                </Grid>
              </Fragment>
            )}
          </Grid>
        </Box>
        <Box
          sx={{
            height: "2px",
            background: SEVERITIES[severity].color,
            mt: "-2px",
          }}
        />
      </Box>
    </MuiSnackbar>
  );
};
