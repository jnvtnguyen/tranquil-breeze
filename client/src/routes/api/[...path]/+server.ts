import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ params, url, request }) => {
	return fetch(`http://localhost:5004/api/${params.path + url.search}`, {
		method: 'GET',
		headers: request.headers,
		body: request.body,
		//@ts-ignore
		duplex: 'half'
	});
};

export const POST: RequestHandler = async ({ params, url, request }) => {
	return fetch(`http://localhost:5004/api/${params.path + url.search}`, {
		method: 'POST',
		headers: request.headers,
		body: request.body,
		//@ts-ignore
		duplex: 'half'
	});
};
