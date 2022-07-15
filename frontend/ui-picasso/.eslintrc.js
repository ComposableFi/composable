module.exports = {
  parser: "@typescript-eslint/parser",
  extends: [
    "next/core-web-vitals",
    "prettier",
    "plugin:storybook/recommended",
    "plugin:prettier/recommended",
    "plugin:@next/next/recommended",
  ],
  plugins: ["import"],
  rules: {
    "import/no-unresolved": 0,
    "react/no-danger": 0,
    "prettier/prettier": [
      "error",
      {
        endOfLine: "auto",
      },
    ],
  },
};
