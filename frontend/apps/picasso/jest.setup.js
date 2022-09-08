// Optional: configure or set up a testing framework before each test.
// If you delete this file, remove `setupFilesAfterEnv` from `jest.config.js`

// Used for __tests__/testing-library.js
// Learn more: https://github.com/testing-library/jest-dom
import "@testing-library/jest-dom/extend-expect";
import { loadEnvConfig } from "@next/env";
import * as globalConfig from "../picasso-storybook/.storybook/preview";
import { setGlobalConfig } from "@storybook/testing-react";

const loadEnvironments = () => loadEnvConfig(process.cwd());

loadEnvironments();
setGlobalConfig(globalConfig);
jest.mock("bi-lib", () => ({
  ConnectorType: {
    MetaMask: "metamask",
    Static: "static",
  },
  useBlockchainProvider: jest.fn(() => ({
    account: "0x0000000000000000000000000000000000000000",
  })),
  useConnector: jest.fn((connector) => ({
    isActive: false,
    connector,
  })),
}));
jest.mock("notistack", () => ({
  useSnackbar: jest.fn().mockImplementation(() => ({
    closeSnackbar: jest.fn(),
    enqueueSnackbar: jest.fn(),
  })),
}));

jest.mock("react-apexcharts", () => {
  return {
    __esModule: true,
    default: () => {
      return <div />;
    },
  };
});

jest.isolateModules(() => {
  const preloadAll = require("jest-next-dynamic");
  beforeAll(async () => {
    await preloadAll();
  });
});
