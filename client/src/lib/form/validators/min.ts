import type { Validator } from '../types';

export function min(min: number): Validator {
	return (value: any) => {
		const v = isNaN(value) ? value.length : parseFloat(value);
		return { valid: v >= min, name: `min.${min}` };
	};
}
