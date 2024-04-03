// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/duotoneLight')
const darkCodeTheme = require('prism-react-renderer/themes/duotoneDark')
const math = require('remark-math')
const katex = require('rehype-katex')

/** @type {import('@docusaurus/types').Config} */
const config = {
	title: 'Picasso Documentation',
	tagline: 'The Cross-Ecosystem IBC & Restaking Hub',
	url: 'https://docs.picasso.network',
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
				logo: {
					alt: 'Picasso Logo',
					src: 'img/picasso-dark.svg',
					srcDark: 'img/picasso-light.svg',

				},
				items: [
					{
						to: '/concepts/picasso',
						position: 'left',
						label: 'Concepts',
					},
					{
						to: '/technology/ibc',
						position: 'left',
						label: 'IBC',
					},
					{
						to: '/technology/restaking',
						position: 'left',
						label: 'Restaking',
					},
					{
						to: '/governance-&-token/use-cases',
						position: 'left',
						label: 'Governance & Token',
					},
					{
						to: '/user-guides',
						position: 'left',
						label: 'Guides',
					},
					{
						to: '/technology/mantis',
						position: 'left',
						label: 'MANTIS',
					},
					{
						href: 'https://research.composable.finance',
						label: 'Research',
						position: 'right',
					},
					{
						href: 'https://github.com/ComposableFi/composable',
						label: 'GitHub',
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
								label: 'Twitter',
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
								label: 'Medium',
								href: 'https://medium.com/@picasso_network',
							},
							{
								label: 'Press Kit',
								href: 'https://docs.composable.finance/ecosystem/press-kit',
							},
							{
								label: 'Risk Factors',
								href: 'https://docs.composable.finance/faqs/risk-factors',
							},
							{
								label: 'Terms of Use',
								href: 'https://docs.composable.finance/faqs/terms-of-use',
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