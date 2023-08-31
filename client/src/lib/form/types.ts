export type FieldOptions = {
	valid: boolean;
	validateOnChange: boolean;
	validateOnBlur: boolean;
	stopAtFirstError: boolean;
	isOptional: boolean;
};

export type FieldMeta = {
	valid: boolean;
	dirty: boolean;
};

export type Field<T> = {
	name: string;
	title: string;
	value: T;
	invalid: boolean;
	errors: string[];
	meta: FieldMeta | Partial<FieldMeta>;
};

export type Form = {
	valid: boolean;
	dirty: boolean;
	errors: string[];
};

export type Validation = { valid: boolean; name: string };

export type Validator = (value: any) => Validation | Promise<Validation>;
