import { Link } from "@/components";
import { alpha, Box, Typography, useTheme } from "@mui/material";

export const CrowdloanRewardsSoon = () => {
  const theme = useTheme();

  return (
    <Box
      display="flex"
      alignItems="center"
      justifyContent="center"
      mt={3}
      width="100%"
    >
      <Box
        sx={{
          padding: theme.spacing(2.25, 4),
          backgroundColor: alpha(theme.palette.common.white, 0.1),
          borderRadius: theme.spacing(1.5),
          width: "100%",
        }}
      >
        <Typography variant="body2" textAlign="center">
          Crowdloan rewards will be viewable but non-transferable until Release
          2.
          <br />
          For more information please see{" "}
          <Link target="_blank" href="https://docs.composable.finance">
            docs.composable.finance
          </Link>
        </Typography>
      </Box>
    </Box>
  );
};
