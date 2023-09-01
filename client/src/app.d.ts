import type { User } from './types';

declare global {
	namespace App {
		interface Locals {
			user: User;
		}
	}
}

export {};
