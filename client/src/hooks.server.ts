import { redirect, type Handle } from '@sveltejs/kit';

const UNPROTECTED_ROUTES = ['/login', '/signup'];
const UNPROTECTED_API_ROUTES = ['/api/login', '/api/signup', '/api/user'];

export const handle: Handle = async ({ event, resolve }): Promise<Response> => {
	const token = event.cookies.get('token');

	if (!token && !UNPROTECTED_ROUTES.includes(event.url.pathname)) {
		throw redirect(303, '/login');
	}
	if (token && !UNPROTECTED_API_ROUTES.includes(event.url.pathname)) {
		const bearer = `Bearer ${token}`;

		const user = await event.fetch('/api/user', {
			method: 'POST',
			headers: {
				Authorization: bearer
			}
		});

		if (user.status === 401) {
			await event.cookies.delete('token');
			throw redirect(303, '/login');
		}

		if (UNPROTECTED_ROUTES.includes(event.url.pathname)) {
			throw redirect(303, '/workspaces');
		}

		if (
			event.url.pathname.startsWith('/api') &&
			!UNPROTECTED_API_ROUTES.includes(event.url.pathname)
		) {
			console.log(event.url.pathname);
			event.request.headers.append('Authorization', bearer);
			return await resolve(event);
		}

		const data = await user.json();
		event.locals.user = data.user;

		return await resolve(event);
	}

	return await resolve(event);
};
