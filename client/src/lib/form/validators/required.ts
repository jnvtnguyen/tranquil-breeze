import type { Validator } from '../types';

export function required(): Validator {
	return (v: string) => {
		let valid = true;
		if (v === undefined || v === null) valid = false;

		if (typeof v === 'string') {
			const t = v.replace(/\s/g, '');

			valid = t.length > 0;
		}

		return { valid, name: 'required' };
	};
}
