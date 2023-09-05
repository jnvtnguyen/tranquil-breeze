import type { RequestHandler } from '@sveltejs/kit';
import { API_HOST } from '$env/static/private';

export const POST: RequestHandler = async ({ request, cookies, fetch }) => {
	const response = await fetch(`${API_HOST}/api/users/signup`, {
		method: 'POST',
		headers: request.headers,
		body: request.body,
		//@ts-ignore
		duplex: 'half'
	});

	let body = null;

	if (response?.ok) {
		body = await response.json();
		cookies.set('token', body.user.token, {
			path: '/',
			sameSite: 'strict',
			httpOnly: true
		});
	}

	return new Response(body, {
		status: response.status,
		headers: response.headers
	});
};
