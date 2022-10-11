import { Theme } from "@mui/material";

export const typographyOptions = (theme: Theme) => ({
  fontFamily: `"${theme.custom.fontFamily.primary}", "${theme.custom.fontFamily.secondary}", sans-serif`,
  htmlFontSize: 16,
  h1: {
    fontFamily: `"${theme.custom.fontFamily.secondary}"`,
    lineHeight: theme.custom.lineHeight.small,
    fontSize: "6rem",
    fontWeight: "normal",
    letterSpacing: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "4.5rem",
    },
  },
  h2: {
    fontFamily: `"${theme.custom.fontFamily.secondary}"`,
    lineHeight: theme.custom.lineHeight.small,
    fontSize: "4.5rem",
    fontWeight: "normal",
    letterSpacing: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "4rem",
    },
  },
  h3: {
    fontFamily: `"${theme.custom.fontFamily.secondary}"`,
    lineHeight: theme.custom.lineHeight.large,
    fontSize: "4rem",
    fontWeight: "normal",
    letterSpacing: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "3rem",
    },
  },
  h4: {
    fontFamily: `"${theme.custom.fontFamily.secondary}"`,
    lineHeight: theme.custom.lineHeight.large,
    fontSize: "3rem",
    fontWeight: "normal",
    letterSpacing: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "2rem",
    },
  },
  h5: {
    fontFamily: `"${theme.custom.fontFamily.primary}"`,
    lineHeight: theme.custom.lineHeight.larger,
    fontSize: "2rem",
    fontWeight: "normal",
    letterSpacing: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "1.5rem",
    },
  },
  h6: {
    fontFamily: `"${theme.custom.fontFamily.primary}"`,
    lineHeight: theme.custom.lineHeight.larger,
    fontSize: "1.5rem",
    fontWeight: "normal",
    letterSpacing: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "1.25rem",
    },
  },
  subtitle1: {
    fontFamily: `"${theme.custom.fontFamily.primary}"`,
    lineHeight: theme.custom.lineHeight.larger,
    fontSize: "1.25rem",
    fontWeight: 400,
    letterSpacing: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "1.125rem",
    },
  },
  subtitle2: {
    fontFamily: `"${theme.custom.fontFamily.primary}"`,
    lineHeight: theme.custom.lineHeight.larger,
    fontSize: "1rem",
    fontWeight: "normal",
    letterSpacing: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "1rem",
    },
  },
  body1: {
    fontFamily: `"${theme.custom.fontFamily.primary}"`,
    lineHeight: theme.custom.lineHeight.larger,
    fontSize: "1.125rem",
    fontWeight: 300,
    letterSpacing: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "1rem",
    },
  },
  body2: {
    fontFamily: `"${theme.custom.fontFamily.primary}"`,
    lineHeight: theme.custom.lineHeight.larger,
    fontSize: "1rem",
    fontWeight: 300,
    letterSpacing: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "1rem",
    },
  },
  button: {
    fontFamily: `"${theme.custom.fontFamily.primary}"`,
    lineHeight: theme.custom.lineHeight.large,
    fontSize: "1.125rem",
    width: "max-content",
    fontWeight: 400,
    letterSpacing: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "1rem",
    },
  },
  caption: {
    fontFamily: `"${theme.custom.fontFamily.primary}"`,
    lineHeight: theme.custom.lineHeight.larger,
    fontSize: "0.75rem",
    fontWeight: 300,
    letterSpacing: "normal",
    [theme.breakpoints.down("sm")]: {
      fontSize: "0.625rem",
    },
  },
  inputLabel: {
    fontFamily: `"${theme.custom.fontFamily.primary}"`,
    lineHeight: theme.custom.lineHeight.larger,
    fontSize: "1rem",
    fontWeight: 300,
    letterSpacing: "normal",
  },
});
