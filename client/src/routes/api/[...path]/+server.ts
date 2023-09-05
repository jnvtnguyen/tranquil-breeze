import type { RequestHandler } from './$types';
import { API_HOST } from '$env/static/private';

export const GET: RequestHandler = async ({ params, url, request }) => {
	return fetch(`${API_HOST}/api/${params.path + url.search}`, {
		method: 'GET',
		headers: request.headers,
		body: request.body,
		//@ts-ignore
		duplex: 'half'
	});
};

export const POST: RequestHandler = async ({ params, url, request }) => {
	return fetch(`${API_HOST}/api/${params.path + url.search}`, {
		method: 'POST',
		headers: request.headers,
		body: request.body,
		//@ts-ignore
		duplex: 'half'
	});
};
