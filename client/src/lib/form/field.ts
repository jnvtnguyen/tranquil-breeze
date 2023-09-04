import type { Readable } from 'svelte/store';
import { writable, get } from 'svelte/store';
import isPromise from 'is-promise';
import type { Field, Validator, Validation, FieldOptions, FieldMeta } from './types';

function isField<T>(field: any): field is Field<T> {
	const keys = Object.keys(field);
	return ['name', 'value', 'valid', 'invalid', 'errors'].every((key) => keys.includes(key));
}

function process<T>(
	field: Field<T>,
	validations?: Validation[],
	meta: Partial<FieldMeta> = {},
	partial: Partial<Field<T>> = {}
): Field<T> {
	if (validations) {
		const errors = validations.filter((v) => !v.valid).map((v) => v.name);
		const valid = !errors.length;

		return { ...field, meta: { valid: valid, ...meta }, invalid: !valid, errors, ...partial };
	}

	return field;
}

function getValue<T>(value: T | Field<T> | Readable<Field<T>>): T {
	const isStore = (v: T | Field<T> | Readable<Field<T>>): v is Readable<Field<T>> => {
		return (value as Readable<Field<T>>).subscribe !== undefined;
	};

	const isField = (v: T | Field<T> | Readable<Field<T>>): v is Field<T> => {
		return !!(value as Field<T>).name && (value as Field<T>).meta.valid !== undefined;
	};

	if (isStore(value)) {
		return get(value).value;
	} else if (isField(value)) {
		return value.value;
	}

	return value;
}

async function getErrors<T>(
	value: T | Field<T> | Readable<Field<T>>,
	validators: Validator[],
	stopAtAFirstError: boolean = false,
	isOptional = false
) {
	const v = getValue(value);

	if (isOptional && !v) {
		return [];
	}

	let errors: Validation[] = [];

	for (const validator of validators) {
		let result = validator(v);

		if (isPromise(result)) {
			result = await result;
		}

		if (stopAtAFirstError && !result.valid) {
			errors = [result];
			break;
		}

		errors = [...errors, result];
	}

	return errors;
}

export function field<T>(
	name: string,
	value: T,
	title?: string,
	validators: Validator[] = [],
	partial: Partial<FieldOptions> = {}
) {
	const options: FieldOptions = {
		valid: true,
		validateOnChange: true,
		stopAtFirstError: true,
		isOptional: false,
		...partial
	};

	const field: Field<T> = {
		name,
		value,
		title: title || name,
		invalid: !options.valid,
		errors: [],
		meta: {
			valid: options.valid,
			dirty: false
		}
	};

	const store = writable<Field<T>>(field);
	const { subscribe, set: _set, update } = store;

	async function set(this: void, field: Field<T> | T, force: boolean = false) {
		if (!isField(field)) {
			field = process(get(store), [], {}, { value: getValue(field) });
		}

		if (force || options.validateOnChange) {
			let validations = await getErrors(
				field,
				validators,
				options.stopAtFirstError,
				options.isOptional
			);
			_set(process(field, validations, { dirty: true }));
		} else {
			_set(process(field, null, { dirty: true }));
		}
	}

	async function validate() {
		const errors = await getErrors(store, validators, options.stopAtFirstError, options.isOptional);
		let f: Field<T>;

		update((field) => {
			f = process(field, errors, { dirty: true });
			return f;
		});

		return f;
	}

	function reset() {
		_set(
			process({
				errors: [],
				name,
				title: title || name,
				invalid: !options.valid,
				value: value,
				meta: {
					valid: options.valid,
					dirty: false
				}
			})
		);
	}

	function clear() {
		_set(
			process({
				errors: [],
				name,
				title: title || name,
				invalid: !options.valid,
				value: null,
				meta: {
					valid: options.valid,
					dirty: false
				}
			})
		);
	}

	return { subscribe, set, update, validate, reset, clear };
}
