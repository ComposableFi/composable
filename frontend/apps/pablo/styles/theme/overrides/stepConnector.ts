import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const stepConnectorOverrides = (
  theme: Theme
): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      top: 15,
      left: "calc(-50% + 17px)",
      right: "calc(50% + 17px)",
      "&.Mui-active, &.Mui-completed": {
        "& .MuiStepConnector-line": {
          borderTopStyle: 'solid',
          borderTopWidth: 4,
        },
      },
    },
    line: {
      borderTopStyle: 'dotted',
      borderColor: theme.palette.text.secondary,
    }
  },
});
