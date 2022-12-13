import { Link } from "@/components";
import { alpha, Box, Typography, useTheme } from "@mui/material";

type Props = {
  pageTitle: string;
};

export const UnavailableFeature = ({ pageTitle }: Props) => {
  const theme = useTheme();

  return (
    <Box
      display="flex"
      alignItems="center"
      justifyContent="center"
      mt={3}
      sx={{ width: "100%" }}
    >
      <Box
        sx={{
          padding: theme.spacing(2.25, 4),
          backgroundColor: alpha(theme.palette.secondary.light, 0.2),
          borderRadius: theme.spacing(1.5),
          width: "100%",
        }}
      >
        <Typography variant="body2" textAlign="center">
          {pageTitle} will be available soon. For more information see{" "}
          <Link
            target="_blank"
            href="https://docs.composable.finance"
            sx={{
              display: "inline",
            }}
          >
            docs.composable.finance
          </Link>
        </Typography>
      </Box>
    </Box>
  );
};
