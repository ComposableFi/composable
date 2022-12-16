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
  testPathIgnorePatterns: [
    "/node_modules/",
     // Enable once fixed.
     "/tests/atoms/network_select.test.tsx",
     "tests/organisms/tabs.test.tsx",
     "/tests/organisms/statsOverviewTab.test.tsx",
     "/tests/atoms/search_input.test.tsx",
     "/tests/organisms/statsGovernanceTab.test.tsx",
     "/tests/atoms/token_select.test.tsx",
     "/tests/molecules/recipient_dropdown.test.tsx",
     "/tests/integration/pallets/stakingRewards.test.ts",
     "/tests/organisms/network_tabs.test.tsx",
     "/tests/molecules/featured_box.test.tsx",
     "/tests/integrations/math.test.tsx",
     "/tests/atoms/label.test.tsx",
     "/tests/organisms/bond_buy_single_token_modal.test.tsx",
     "/tests/molecules/votingDetailsBox.test.tsx",
     "/tests/molecules/page_title.test.tsx",
     "/tests/molecules/token_dropdown_combined_input.test.tsx",
     "/tests/molecules/fee_display.test.tsx",
     "/tests/atoms/network_asset.test.tsx",
     "/tests/organisms/pool_statistics.test.tsx",
     "/tests/organisms/overview_wallet_breakdown_box.test.tsx",
     "/tests/organisms/pool_details.test.tsx",
     "/tests/atoms/select.test.tsx",
     "/tests/molecules/page_title.test.tsx",
     "/tests/organisms/staking_renew_modal.test.tsx",
     "/tests/molecules/textSwitch.test.tsx",
     "/tests/pages/home.test.tsx",
     "/organisms/staking_unstake_modal.test.tsx", 
     "/organisms/staking_checkable_xpablo_item_box.test.tsx"
  ]
};
