import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const cssBaselineOverrides = (
  theme: Theme
): Partial<OverridesStyleRules> => ({
  styleOverrides: `
    @font-face {
      font-family: "CodeNext";
      font-style: normal;
      font-display: swap;
      font-weight: normal;
      src: local('CodeNext-Regular'), local('CodeNext-Regular'), url("/static/CodeNext-Regular.woff2") format('woff2');
    };
    @font-face {
      font-family: "CodeNext";
      font-style: normal;
      font-display: swap;
      font-weight: 600;
      src: local('CodeNext-Bold'), local('CodeNext-Bold'), url("/static/CodeNext-Bold.woff2") format('woff2');
    };
    @font-face {
      font-family: "NovaDeco";
      font-style: normal;
      font-display: swap;
      font-weight: normal;
      src: local('NovaDeco-Medium'), local('NovaDeco-Medium'), url("/static/NovaDeco-Medium.woff2") format('woff2');
    };
    @font-face {
      font-family: "NovaDeco";
      font-style: normal;
      font-display: swap;
      font-weight: 300;
      src: local('NovaDeco-Medium'), local('NovaDeco-Medium'), url("/static/NovaDeco-Medium.woff2") format('woff2');
    };
    @font-face {
      font-family: "NovaDeco";
      font-style: normal;
      font-display: swap;
      font-weight: 600;
      src: local('NovaDeco-Bold'), local('NovaDeco-Bold'), url("/static/NovaDeco-Bold.woff2") format('woff2');
    };
    body {
      background: ${theme.palette.gradient.grapeBackground};
      min-height: 100vh;
    };
    * {
      user-select: none;
    }
    /* width */
    div::-webkit-scrollbar {
      width: 16px;
    };

    /* Track */
    div::-webkit-scrollbar-track {
      background: ${alpha(
        theme.palette.common.white,
        theme.custom.opacity.light
      )};
      border-radius: 0 0px 12px 0;
    };

    /* Handle */
    div::-webkit-scrollbar-thumb {
      background: ${theme.palette.common.white};
      border: 7px solid rgb(40 38 56);
      border-radius: 12px;
    };

    /* Handle on hover */
    div::-webkit-scrollbar-thumb:hover {
      background: ${theme.palette.common.white};
    };
  `,
});
