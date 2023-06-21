import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const switchOverrides = (
  theme: Theme
): Partial<OverridesStyleRules> => ({
  styleOverrides: {
    root: {
      width: theme.spacing(8),
      height: theme.spacing(4),
      borderRadius: theme.shape.borderRadius,
      padding: 0,
      border: `1px solid ${alpha(
        theme.palette.common.white,
        theme.custom.opacity.light
      )}`,
      "& .MuiSwitch-switchBase": {
        padding: 0,
        marginTop: theme.spacing(0.25),
        paddingLeft: theme.spacing(0.5),
        transitionDuration: "200ms",
        "&.Mui-checked": {
          transform: `translateX(${theme.spacing(4)})`,
          paddingLeft: 0,
          color: theme.palette.common.white,
          "& + .MuiSwitch-track": {
            backgroundColor: theme.palette.primary.light,
            opacity: 1,
            border: 0,
          },
          "&.Mui-disabled + .MuiSwitch-track": {
            opacity: theme.custom.opacity.dark,
          },
        },
        "&.Mui-disabled": {
          "& .MuiSwitch-thumb": {
            color: alpha(theme.palette.common.white, theme.custom.opacity.main),
          },
        },
        "&.Mui-disabled + .MuiSwitch-track": {
          opacity: theme.custom.opacity.dark,
        },
      },
      "& .MuiSwitch-thumb": {
        boxSizing: "border-box",
        width: theme.spacing(3),
        height: theme.spacing(3),
        color: theme.palette.common.white,
      },
      "& .MuiSwitch-track": {
        backgroundColor: "transparent",
        opacity: 1,
        transition: theme.transitions.create(["background-color"], {
          duration: 300,
        }),
      },
    },
  },
});
