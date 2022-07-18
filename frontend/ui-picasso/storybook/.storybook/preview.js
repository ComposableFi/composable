import { RouterContext } from "next/dist/shared/lib/router-context";
import * as NextImage from "next/image";
import { MUIDecorator } from "./mui-decorator";
import { StorybookRouterProvider } from "./router-decorator";

const OriginalNextImage = NextImage.default;

Object.defineProperty(NextImage, "default", {
  configurable: true,
  value: (props) => (
      <OriginalNextImage {...props} unoptimized loader={({ src }) => src} />
  ),
});
export const decorators = [
  (Story) => (
      <StorybookRouterProvider>
        <MUIDecorator>
          <Story />
        </MUIDecorator>
      </StorybookRouterProvider>
  ),
];

export const parameters = {
  actions: { argTypesRegex: "^on[A-Z].*" },
  controls: {
    matchers: {
      color: /(background|color)$/i,
      date: /Date$/,
    },
  },
  backgrounds: {
    default: "dark",
    values: [
      {
        name: "dark",
        value: "#000",
      },
      {
        name: "light",
        value: "#fff",
      },
    ],
  },
  nextRouter: {
    Provider: RouterContext.Provider,
  },
};
