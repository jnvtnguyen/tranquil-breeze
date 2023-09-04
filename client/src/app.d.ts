import type { User, Workspace } from '$lib/types';

declare global {
	namespace App {
		interface Locals {
			user: User;
			workspace: Workspace;
		}
	}
}

export {};
