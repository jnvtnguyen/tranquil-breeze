import type { SvelteComponent } from 'svelte';
//@ts-ignore
import HomeIcon from '~icons/mdi/home';
//@ts-ignore
import SettingsIcon from '~icons/mdi/cog';

export type LinkData = {
	title: string;
	href: string;
	icon: typeof SvelteComponent;
};

export const LINKS: LinkData[] = [
	{
		title: 'Projects Home',
		href: '',
		icon: HomeIcon
	},
	{
		title: 'Settings',
		href: '/settings',
		icon: SettingsIcon
	}
];
