import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, type UserConfig } from 'vite';

export default ({ mode }: UserConfig) => {
	let dev = {};
	if (mode === 'development') {
		dev = {
			server: {
				port: 3000,
				proxy: {
					'/api': 'http://localhost:5004'
				}
			}
		};
	}

	return defineConfig({
		plugins: [sveltekit()],
		...dev
	});
};
