import type { Action } from 'svelte/action';

const selectorTabbable = `
  a[href], area[href], input:not([disabled]):not([tabindex='-1']),
  button:not([disabled]):not([tabindex='-1']),select:not([disabled]):not([tabindex='-1']),
  textarea:not([disabled]):not([tabindex='-1']),
  iframe, object, embed, *[tabindex]:not([tabindex='-1']):not([disabled]), *[contenteditable=true]
`;

export default function focusTrap(node: any) {
	const handleFocusTrap = (e: KeyboardEvent) => {
		let isTabPressed = e.key === 'Tab' || e.keyCode === 9;

		if (!isTabPressed) {
			return;
		}

		const tabbable = Array.from(node.querySelectorAll(selectorTabbable));

		let index = tabbable.indexOf(document.activeElement ?? node);
		if (index === -1 && e.shiftKey) index = 0;
		index += tabbable.length + (e.shiftKey ? -1 : 1);
		index %= tabbable.length;
		/** @ts-ignore */
		tabbable[index].focus();

		e.preventDefault();
	};

	document.addEventListener('keydown', handleFocusTrap, true);

	return {
		destroy() {
			document.removeEventListener('keydown', handleFocusTrap, true);
		}
	};
}
