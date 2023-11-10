// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/duotoneLight')
const darkCodeTheme = require('prism-react-renderer/themes/duotoneDark')
const math = require('remark-math')
const katex = require('rehype-katex')

/** @type {import('@docusaurus/types').Config} */
const config = {
	title: 'Composable Finance',
	tagline: 'The interoperable infrastructure for Modular DeFi',
	url: 'https://composable.finance',
	baseUrl: '/',
	onBrokenLinks: 'warn',
	onBrokenMarkdownLinks: 'warn',
	favicon: 'img/favicon.ico',

	// GitHub pages deployment config.
	// If you aren't using GitHub pages, you don't need these.
	organizationName: 'Composable Finance', // Usually your GitHub org/user name.
	projectName: 'composable', // Usually your repo name.

	// Even if you don't use internalization, you can use this field to set useful
	// metadata like html lang. For example, if your site is Chinese, you may want
	// to replace "en" with "zh-Hans".
	i18n: {
		defaultLocale: 'en',
		locales: ['en'],
	},

	presets: [
		[
			'@docusaurus/preset-classic',
			/** @type {import('@docusaurus/preset-classic').Options} */
			({
				docs: {
					breadcrumbs: true,
					sidebarPath: require.resolve('./sidebars.js'),
					routeBasePath: '/',
					editUrl: 'https://github.com/ComposableFi/composable/tree/main/docs/',
					remarkPlugins: [math],
					rehypePlugins: [katex]
				},
				blog: false,
				theme: {
					customCss: require.resolve('./src/css/custom.css'),
				},
			}),
		],
	],
	stylesheets: [
		{
		  href: 'https://cdn.jsdelivr.net/npm/katex@0.13.24/dist/katex.min.css',
		  type: 'text/css',
		  integrity:
			'sha384-odtC+0UGzzFL/6PNoE8rX/SPcQDXBJ+uRepguP4QkPCm2LBxH3FA3y+fKSiJ+AmM',
		  crossorigin: 'anonymous',
		},
	  ],	

	themeConfig:
		/** @type {import('@docusaurus/preset-classic').ThemeConfig} */
		({
			algolia: {
				// The application ID provided by Algolia
				appId: '1GMXVIRCBW',
				// Public API key: it is safe to commit it
				apiKey: 'de939a9de56cd5e30ef4a25b9f61a641',
				indexName: 'composable',
			},
			navbar: {
				title: 'Composable Finance',
				logo: {
					alt: 'Composable Finance Logo',
					src: 'img/logo.svg',
				},
				items: [
					{
						to: '/networks/picasso-parachain-overview',
						position: 'left',
						label: 'Networks',
					},
					{
						to: '/technology/ibc',
						position: 'left',
						label: 'Architecture',
					},
					{
						to: '/develop/build-on-composable',
						position: 'left',
						label: 'Develop',
					},
					{
						to: '/ecosystem/composable-ecosystem',
						position: 'left',
						label: 'Ecosystem',
					},
					{
						to: '/user-guides',
						position: 'left',
						label: 'User Guides',
					},
					{
						href: 'https://github.com/ComposableFi/composable',
						label: 'GitHub',
						position: 'right',
					},
					{
						href: 'https://explorer.trustless.zone/',
						label: 'Explorer',
						position: 'right',
					},
					{
						href: 'https://composablefi.medium.com/',
						label: 'Medium',
						position: 'right',
					},
				],
			},
			footer: {
				style: 'dark',
				links: [
					{
						title: 'Community',
						items: [
							{
								label: 'Composable Twitter',
								href: 'https://twitter.com/composablefin',
							},
							{
								label: 'Picasso Twitter',
								href: 'https://twitter.com/Picasso_Network',
							},
							{
								label: 'Telegram',
								href: 'https://t.me/https://t.me/composablefinance',
							},
							{
								label: 'Discord',
								href: 'https://discord.gg/composable',
							},
							{
								label: 'LinkedIn',
								href: 'https://www.linkedin.com/company/composable-finance/',
							},
						],
					},
					{
						title: 'More',
						items: [
							{
								label: 'GitHub',
								href: 'https://github.com/ComposableFi/composable',
							},
							{
								label: 'Composable Medium',
								href: 'https://composablefi.medium.com',
							},
							{
								label: 'Picasso Medium',
								href: 'https://medium.com/@picasso_network',
							},
							{
								label: 'Press Kit',
								href: 'https://docs.composable.finance/ecosystem/press-kit',
							},
						],
					},
				],
				copyright: `Copyright © ${new Date().getFullYear()} Composable Foundation`,
			},
			prism: {
				additionalLanguages: ['rust', 'haskell', 'nix', 'typescript', 'tsx', 'toml', 'yaml', 'json', 'bash'],
				theme: lightCodeTheme,
				darkTheme: darkCodeTheme,
			},
		}),
	plugins: ['docusaurus-plugin-sass', 'my-loaders'],
	markdown: {
		mermaid: true,
	},
	themes: ['@docusaurus/theme-mermaid'],
};

module.exports = config;