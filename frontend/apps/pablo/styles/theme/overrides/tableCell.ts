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
      fontSize: "1.125rem",
    },
    body: {
      border: "none",
      background: alpha(theme.palette.common.white, 0.05),
      padding: theme.spacing(2),
      "&:first-of-type": {
        borderTopLeftRadius: theme.shape.borderRadius,
        borderBottomLeftRadius: theme.shape.borderRadius,
      },
      "&:last-of-type": {
        borderTopRightRadius: theme.shape.borderRadius,
        borderBottomRightRadius: theme.shape.borderRadius,
      },
    },
  },
});
