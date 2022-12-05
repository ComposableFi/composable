/** @type {import('tailwindcss').Config} */
module.exports = {
	jit: true,
	content: ['./src/**/*.{js,jsx,ts,tsx}', './docs/**/*.mdx'],
	darkMode: 'media',
	theme: {
		extend: {},
	},
	plugins: [
		async function myPlugin(context, options) {
			return {
				name: 'docusaurus-tailwindcss',
				configurePostCss(postcssOptions) {
					// Appends TailwindCSS and AutoPrefixer.
					postcssOptions.plugins.push(require('tailwindcss'));
					postcssOptions.plugins.push(require('autoprefixer'));
					return postcssOptions;
				},
			};
		},
	],
};
