import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, type UserConfig } from 'vite';
import path from 'path';
import Icons from 'unplugin-icons/vite';

export default ({ mode }: UserConfig) => {
	let dev = {};
	if (mode === 'development') {
		dev = {
			server: {
				port: 3000
			}
		};
	}

	return defineConfig({
		plugins: [
			sveltekit(),
			Icons({
				compiler: 'svelte',
				autoInstall: true
			})
		],
		resolve: {
			alias: {
				$lib: path.resolve('./src/lib')
			}
		},
		...dev
	});
};
