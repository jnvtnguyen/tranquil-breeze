import type { Readable, Writable } from 'svelte/store';
import { derived, get } from 'svelte/store';
import type { Field } from './types';

export function form(...fields: (Writable<Field<any>> | Readable<Field<any>>)[]) {
	const store = derived(fields, (values) => {
		return {
			valid: values.every((v) => v.meta.valid),
			dirty: values.some((v) => v.meta.dirty),

			get summary() {
				return values.reduce((a: any, v: Field<any>) => {
					a[v.name] = v.value;

					return a;
				}, {});
			},

			errors: values.map((v) => {
				return v.errors.map((e) => {
					if (e.includes('.')) {
						return e;
					}

					return `${v.name}.${e}`;
				});
			})
		};
	});

	const { subscribe } = store;

	function reset() {
		fields.forEach((field: any) => field.reset && field.reset());
	}

	function clear() {
		fields.forEach((field: any) => field.clear && field.clear());
	}

	async function validate() {
		for (const field of fields as any[]) {
			if (field.validate) await field.validate();
		}
	}

	function getField(name: string): Writable<Field<any>> | Readable<Field<any>> | undefined {
		return fields.find((f) => get(f).name === name);
	}

	function summary(): Record<string, any> {
		return get(store).summary;
	}

	return { subscribe, reset, clear, validate, getField, summary };
}
