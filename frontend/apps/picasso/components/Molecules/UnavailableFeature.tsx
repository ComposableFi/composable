import { Link } from "@/components";
import { alpha, Box, Typography, useTheme } from "@mui/material";

type Props = {
  pageTitle: string;
};
export const UnavailableFeature = ({ pageTitle }: Props) => {
  const theme = useTheme();

  return (
    <Box display="flex" alignItems="center" justifyContent="center" mt={3}>
      <Box
        sx={{
          padding: theme.spacing(2.25, 4),
          backgroundColor: alpha(theme.palette.common.white, 0.1),
          borderRadius: theme.spacing(1.5),
        }}
      >
        <Typography variant="body2">
          {pageTitle} will be available soon. For more information do check{" "}
          <Link target="_blank" href="https://docs.composable.finance">
            docs.composable.finance
          </Link>
        </Typography>
      </Box>
    </Box>
  );
};
