module.exports = {
    extends: [
        "next",
        "prettier",
        "next/core-web-vitals",
        "plugin:storybook/recommended"
    ],
    rules: {
        "@next/next/no-html-link-for-pages": "off",
        "react/jsx-key": "off",
    },
};
