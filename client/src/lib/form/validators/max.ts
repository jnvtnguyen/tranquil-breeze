import type { Validator } from '../types';

export function max(max: number): Validator {
	return (value: any) => {
		const v = isNaN(value) ? value.length : parseFloat(value);
		return { valid: v <= max, name: `max.${max}` };
	};
}
