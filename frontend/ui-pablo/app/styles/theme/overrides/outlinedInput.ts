import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const outlinedInputOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  defaultProps: {
    notched: false,
  },
  styleOverrides: {
    root: {
      borderRadius: 9999,
      paddingRight: 0,
      background: alpha(theme.palette.common.white, theme.custom.opacity.lighter),
      "&:hover": {
        "& .MuiOutlinedInput-notchedOutline": {
          borderColor: alpha(
            theme.palette.common.white,
            theme.custom.opacity.light
          ),
        },
      },
      "&.Mui-error": {
        color: theme.palette.error.main,
        "&:hover": {
          "& .MuiOutlinedInput-notchedOutline": {
            // borderColor: theme.palette.error.main,
          },
        },
      },
      "&.Mui-focused": {
        color: theme.palette.common.white,
        "& .MuiOutlinedInput-notchedOutline": {
          borderColor: alpha(
            theme.palette.common.white,
            theme.custom.opacity.light
          ),
        },
      },
      "&.Mui-disabled": {
        "& .MuiOutlinedInput-notchedOutline": {
          borderColor: alpha(theme.palette.common.white, theme.custom.opacity.light),
        },
      },
      "& .MuiSelect-icon": {
        right: theme.spacing(3),
        transform: 'none',
        [theme.breakpoints.down("sm")]: {
          right: theme.spacing(2),
        },
      },
      "&.MuiInputBase-adornedStart": {
        paddingLeft: theme.spacing(3),
        [theme.breakpoints.down("sm")]: {
          paddingLeft: theme.spacing(2),
        },
      },
      "& .MuiSelect-select": {
        "&.MuiOutlinedInput-input.MuiInputBase-input": {
          paddingRight: theme.spacing(6),
        },
        "& .MuiBox-root": {
          overflow: "hidden",
        },
      },
    },
    input: {
      fontSize: 18,
      lineHeight: 1.55,
      height: "auto",
      padding: theme.spacing(2.25, 3),
      [theme.breakpoints.down("sm")]: {
        fontSize: 16,
        padding: theme.spacing(1.875, 2),
      },
    },
    notchedOutline: {
      borderColor: `${alpha(
        theme.palette.common.white,
        theme.custom.opacity.light
      )}`,
    },
  },
});