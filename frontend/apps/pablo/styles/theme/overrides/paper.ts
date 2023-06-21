import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const paperOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      borderRadius: theme.shape.borderRadius,
      padding: theme.spacing(2),
      backgroundImage: theme.palette.gradient.secondary,
      /* width */
      "&::-webkit-scrollbar": {
        width: 16,
      },

      /* Track */
      "&::-webkit-scrollbar-track": {
        background: alpha(
          theme.palette.common.white,
          theme.custom.opacity.light
        ),
        borderRadius: "0 12px 12px 0",
      },

      /* Handle */
      "&::-webkit-scrollbar-thumb": {
        background: theme.palette.common.white,
        border: "7px solid rgb(40 38 56)",
        borderRadius: theme.shape.borderRadius,
      },

      /* Handle on hover */
      "&::-webkit-scrollbar-thumb:hover": {
        background: theme.palette.common.white,
      },
    },
    outlined: {
      border: `1px solid ${theme.palette.primary.main}`,
      "& img": {
        mixBlendMode: "luminosity",
      },
    },
  },
});
