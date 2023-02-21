import config from "@/constants/config";

import { manager } from "@/utils/Analytics";
import { useProcessorStateStore } from "@/utils/Analytics/manager";
import { ga } from "@/utils/Analytics/Providers/ga";
import { mixPanel } from "@/utils/Analytics/Providers/mixPanel";
import WarningAmberRoundedIcon from "@mui/icons-material/WarningAmberRounded";
import {
  Alert,
  Box,
  Button,
  Collapse,
  Stack,
  Typography,
  useTheme,
} from "@mui/material";
import { useEffect } from "react";
import { Link } from "../Molecules/Link";

const mixPanelProvider = mixPanel(config.analytics.mixpanelToken);
const gaProvider = ga(config.analytics.gaToken);

export function Analytics() {
  const hideConsent = useProcessorStateStore((store) => store.shouldProcess);
  useEffect(() => {
    mixPanelProvider.init();
    manager.addProvider(mixPanelProvider);
    manager.addProvider(gaProvider);
    manager.listen();

    return () => manager.remove();
  }, []);
  const theme = useTheme();
  return (
    <Box
      sx={{
        position: "fixed",
        bottom: theme.spacing(6),
        right: theme.spacing(6),
        display: "flex",
        height: "auto",
      }}
    >
      <Collapse in={!hideConsent}>
        <Alert
          variant="filled"
          color="warning"
          icon={<WarningAmberRoundedIcon color="warning" />}
          sx={{
            maxWidth: "lg",
          }}
        >
          <Stack direction="row" gap={2} alignItems="center">
            <Typography variant="inputLabel">
              This website uses cookies to improve your experience. If you
              continue to use this site, you consent to our use of cookies.
            </Typography>
            <Button
              variant="outlined"
              size="small"
              onClick={() => {
                manager.toggleProcessor(true);
              }}
            >
              OK
            </Button>
            <Link variant="button" href="/privacy-policy">
              Learn more
            </Link>
          </Stack>
        </Alert>
      </Collapse>
    </Box>
  );
}
