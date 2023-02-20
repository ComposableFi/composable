import { Alert, Button, Stack, Typography, useTheme } from "@mui/material";

import { manager } from "@/utils/Analytics";
import { mixPanel } from "@/utils/Analytics/Providers/mixPanel";
import { useEffect } from "react";
import WarningAmberRoundedIcon from "@mui/icons-material/WarningAmberRounded";
import config from "@/constants/config";
import { ga } from "@/utils/Analytics/Providers/ga";
import { Link } from "../Molecules/Link";

const mixPanelProvider = mixPanel(config.analytics.mixpanelToken);
const gaProvider = ga(config.analytics.gaToken);

export function Analytics() {
  useEffect(() => {
    mixPanelProvider.init();
    manager.addProvider(mixPanelProvider);
    manager.addProvider(gaProvider);
    manager.listen();

    return () => manager.remove();
  }, []);
  const theme = useTheme();
  return (
    <Alert
      variant="filled"
      color="warning"
      icon={<WarningAmberRoundedIcon color="warning" />}
      sx={{
        position: "absolute",
        bottom: theme.spacing(2),
        right: theme.spacing(2),
        maxWidth: "lg",
      }}
    >
      <Stack direction="row" gap={2} alignItems="center">
        <Typography variant="inputLabel">
          This website uses cookies to improve your experience. If you continue
          to use this site, you consent to our use of cookies.
        </Typography>
        <Button
          variant="outlined"
          onClick={() => {
            manager.toggleProcessor(true);
          }}
        >
          OK
        </Button>
        <Link href="/privacy-policy">Learn more</Link>
      </Stack>
    </Alert>
  );
}
