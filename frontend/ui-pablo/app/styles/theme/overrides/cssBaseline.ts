import { alpha, Theme } from "@mui/material";
import { OverridesStyleRules } from "@mui/material/styles/overrides";

export const cssBaselineOverrides = (theme: Theme): Partial<OverridesStyleRules> => ({
  styleOverrides: `
    @font-face {
      font-family: "Architects Daughter";
      font-style: normal;
      font-display: swap;
      font-weight: 400;
      src: local('ArchitectsDaughter-Regular'), local('ArchitectsDaughter-Regular'), url("/static/ArchitectsDaughter-Regular.ttf") format('woff');
    };
    @font-face {
      font-family: "BeVietnamPro";
      font-style: normal;
      font-display: swap;
      font-weight: normal;
      src: local('BeVietnamPro-Regular'), local('BeVietnamPro-Regular'), url("/static/BeVietnamPro-Regular.woff2") format('woff');
    };
    @font-face {
      font-family: "BeVietnamPro";
      font-style: normal;
      font-display: swap;
      font-weight: 300;
      src: local('BeVietnamPro-Light'), local('BeVietnamPro-Light'), url("/static/BeVietnamPro-Light.woff2") format('woff');
    };
    @font-face {
      font-family: "BeVietnamPro";
      font-style: normal;
      font-display: swap;
      font-weight: 600;
      src: local('BeVietnamPro-SemiBold'), local('BeVietnamPro-SemiBold'), url("/static/BeVietnamPro-SemiBold.ttf") format('woff');
    };
    body {
      background: ${theme.palette.gradient.background};
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