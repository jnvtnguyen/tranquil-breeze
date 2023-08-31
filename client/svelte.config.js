import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: [vitePreprocess()],
	kit: {
		csp: {
			mode: 'hash',
			directives: {
				'script-src': ['self']
			}
		},
		adapter: adapter({
			fallback: 'index.html'
		})
	}
};

export default config;
