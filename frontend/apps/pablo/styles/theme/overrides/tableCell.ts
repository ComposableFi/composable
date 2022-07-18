import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const tableCellOverrides = (
  theme: Theme
): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    head: {
      color: alpha(theme.palette.common.white, theme.custom.opacity.darker),
      border: "none",
      padding: 0,
      paddingLeft: theme.spacing(2),
      paddingReft: theme.spacing(2),
      fontSize: "1.125rem",
    },
    body: {
      border: "none",
      background: alpha(theme.palette.common.white, 0.05),
      padding: theme.spacing(2),
      "&:first-of-type": {
        borderTopLeftRadius: theme.spacing(2.5),
        borderBottomLeftRadius: theme.spacing(2.5),
      },
      "&:last-of-type": {
        borderTopRightRadius: theme.spacing(2.5),
        borderBottomRightRadius: theme.spacing(2.5),
      },
    },
  },
});
