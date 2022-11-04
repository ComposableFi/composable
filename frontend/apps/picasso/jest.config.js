// jest.config.js
module.exports = {
  collectCoverageFrom: ["**/*.{ts,tsx}", "!**/*.d.ts", "!**/node_modules/**"],
  collectCoverage: false,
  collectCoverageFrom: ["**/*.{ts,tsx}", "**/*.js", "!**/node_modules/**"],
  verbose: true,
  moduleNameMapper: {
    // Handle CSS imports (with CSS modules)
    // https://jestjs.io/docs/webpack#mocking-css-modules
    "^.+\\.module\\.(css|sass|scss)$": "identity-obj-proxy",

    // Handle CSS imports (without CSS modules)
    "^.+\\.(css|sass|scss)$": "<rootDir>/__mocks__/styleMock.js",

    // Handle image imports
    // https://jestjs.io/docs/webpack#handling-static-assets
    "^.+\\.(jpg|jpeg|png|gif|webp|avif)$": `<rootDir>/__mocks__/fileMock.js`,
    "^.+\\.(svg)$": `<rootDir>/__mocks__/svgMock.js`,

    // Handle module aliases
    "^@/(.*)$": "<rootDir>/$1",
    "^assets(.*)$": "<rootDir>/assets$1",
    "^store/(.*)$": "<rootDir>/store$1",
    "^defi/(.*)$": "<rootDir>/defi/$1",
    "^utils(.*)$": "<rootDir>/utils$1",
    "^tests(.*)$": "<rootDir>/tests$1",
  },
  setupFilesAfterEnv: ["<rootDir>/jest.setup.js"],
  testPathIgnorePatterns: ["<rootDir>/node_modules/", "<rootDir>/.next/"],
  testEnvironment: "jsdom",
  preset: "ts-jest/presets/default-esm", // or other ESM presets
  globals: {
    "ts-jest": {
      useESM: true,
    },
  },
  transform: {
    // Use babel-jest to transpile tests with the next/babel preset
    // https://jestjs.io/docs/configuration#transform-objectstring-pathtotransformer--pathtotransformer-object
    "^.+\\.(js|jsx|ts|tsx)$": ["babel-jest", { presets: ["next/babel"] }],
  },
  transformIgnorePatterns: [
    "node_modules/(?!@polkadot|@integrations-lib/core/|@babel/runtime/helpers/esm/|@substrate)",
    "^.+\\.module\\.(css|sass|scss)$",
  ],
};
