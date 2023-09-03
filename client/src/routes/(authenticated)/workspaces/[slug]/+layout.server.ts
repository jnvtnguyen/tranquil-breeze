import type { LayoutServerLoad } from '../$types';
import { error } from '@sveltejs/kit';

export const load: LayoutServerLoad = async ({ params, fetch }) => {
	let response = await fetch(`/api/workspaces/${params.slug}`, {
		method: 'GET'
	});

	if (response?.ok) {
		let data = await response.json();

		return {
			workspace: data.workspace
		};
	} else {
		throw error(404, {
			message: 'Workspace not found.'
		});
	}
};
