import type { LoadEvent } from '@sveltejs/kit';

export const load = ({ url }: LoadEvent) => {
	const { pathname } = url;

	return {
		pathname
	};
};
