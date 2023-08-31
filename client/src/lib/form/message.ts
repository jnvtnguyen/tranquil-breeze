import type { Field } from './types';

export function message(
	field: Field<any>,
	extended?: { error: string; message: string }
): string | null {
	const { errors } = field;

	if (errors.length) {
		const error = errors[0];
		if (error === 'required') {
			return `${field.title} is required.`;
		}
		if (error === 'email') {
			return `${field.title} is not a valid email.`;
		}
		let match = error.match(/^min\.(\d+)$/);
		if (match) {
			return `${field.title} must be at least ${match[1]} characters.`;
		}
		if (extended && error === extended.error) {
			return extended.message;
		}
	}

	return null;
}
