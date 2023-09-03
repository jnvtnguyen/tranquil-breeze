<script lang="ts">
    import type { ComputePositionReturn, Placement, Middleware, Side } from "@floating-ui/dom";
	import { createEventDispatcher, onMount, type ComponentProps } from "svelte";
    import * as dom from "@floating-ui/dom";
    import Frame from "./Frame.svelte";

    interface $$Props extends ComponentProps<Frame> {
        active?: boolean;
        arrow?: boolean;
        offset?: number;
        placement?: Placement;
        trigger?: 'hover' | 'click';
        triggeredBy?: string;
        reference?: string;
        strategy?: 'absolute' | 'fixed';
        open?: boolean;
        yOnly?: boolean;
    }

    export let active: boolean = false;
    export let arrow: boolean = true;
    export let offset: number = 8;
    export let placement: Placement = "top";
    export let trigger: 'hover' | 'click' = 'hover';
    export let triggeredBy: string | undefined = undefined;
    export let reference: string | undefined = undefined;
    export let strategy: 'absolute' | 'fixed' = 'absolute';
    export let open: boolean = false;
    export let yOnly: boolean = false;

    const dispatch = createEventDispatcher();

    $: clickable = trigger === 'click';

    //@ts-ignore
    $: dispatch("show", referenceElement, open)
    $: placement && (referenceElement = referenceElement);

    let referenceElement: Element;
    let floatingElement: HTMLElement;
    let arrowElement: HTMLElement | null;
    let contentElement: HTMLElement;
    let triggerElements: HTMLElement[] = [];

    let _blocked: boolean = false;
    const block = () => ((_blocked = true), setTimeout(() => (_blocked = false), 250));

    const hasHover = (e: Element) => e.matches(':hover');
    const hasFocus = (e: Element) => e.contains(document.activeElement);


    const px = (n: number | undefined) => (n != null ? `${n}px` : '');

    const showHandler = (ev: Event) => {
        if (referenceElement === undefined) console.error('trigger undefined');
        if (!reference && triggerElements.includes(ev.target as HTMLElement) && referenceElement !== ev.target) {
        referenceElement = ev.target as HTMLElement;
        block();
        }
        if (clickable && ev.type === 'focusin' && !open) block();
        open = clickable && ev.type === 'click' && !_blocked ? !open : true;
    };

    const hideHandler = (e: Event) => {
        if (active) {
        setTimeout(() => {
            const elements = [reference, floatingElement, ...triggerElements].filter(Boolean);
            //@ts-ignore
            if (e.type === 'mouseleave' && elements.some(hasHover)) return;
            //@ts-ignore
            if (e.type === 'focusout' && elements.some(hasFocus)) return;
            open = false;
        }, 100);
        } else open = false;
    };

    let middleware: (Middleware | null)[];
    $: middleware = [dom.flip(), dom.shift(), dom.offset(+offset), arrowElement && dom.arrow({ element: arrowElement, padding: 10 })];

    let arrowSide: Side;
    let oppositeSideMap: Record<Side, Side> = {
        top: 'bottom',
        bottom: 'top',
        left: 'right',
        right: 'left'
    };

    const update = () => {
        dom.computePosition(referenceElement, floatingElement, { placement, strategy, middleware }).then(({ x, y, middlewareData, placement, strategy}: ComputePositionReturn) => {
            floatingElement.style.position = strategy;
            floatingElement.style.left = yOnly ? '0' : px(x);
            floatingElement.style.top = px(y);

            if(middlewareData.arrow && arrowElement instanceof HTMLDivElement) {
                arrowElement.style.left = px(middlewareData.arrow.x);
                arrowElement.style.top = px(middlewareData.arrow.y);

                arrowSide = oppositeSideMap[placement.split("-")[0] as Side];
                arrowElement.style[arrowSide] = px(-arrowElement.offsetWidth / 2 - ($$props.border ? 1 : 0));
            }
        });
    }

    const init = (node: HTMLElement, _reference: HTMLElement) => {
        floatingElement = node;
        let cleanup = dom.autoUpdate(_reference, floatingElement, update);

        return {
            update(_reference: HTMLElement) {
                cleanup();
                cleanup = dom.autoUpdate(_reference, floatingElement, update);
            },
            destroy() {
                cleanup();
            }
        }
    }

    const initArrow = (node: HTMLElement) => {
        arrowElement = node;
        return {
            destroy() {
                arrowElement = null;
            }
        }
    }

    function optional(pred: boolean, func: (ev: Event) => void): (ev: Event) => any {
        return pred ? func : () => undefined;
    }

    onMount(() => {
        const events: [string, any, boolean][] = [
            ['focusin', showHandler, true],
            ['focusout', hideHandler, true],
            ['click', showHandler, clickable],
            ['mouseenter', showHandler, !clickable],
            ['mouseleave', hideHandler, !clickable]
        ];

        if (triggeredBy) triggerElements = [...document.querySelectorAll<HTMLElement>(triggeredBy)];
        else triggerElements = contentElement.previousElementSibling ? [contentElement.previousElementSibling as HTMLElement] : [];

        if (!triggerElements.length) {
            console.error('No triggers found.');
        }

        triggerElements.forEach((element: HTMLElement) => {
            if (element.tabIndex < 0) element.tabIndex = 0; 
                for (const [name, handler, cond] of events) if (cond) element.addEventListener(name, handler);
            });

            if (reference) {
                referenceElement = document.querySelector(reference) ?? document.body;
            if (referenceElement === document.body) {
                console.error(`Popup reference not found: '${reference}'`);
            } else {
                referenceElement.addEventListener('focusout', hideHandler);
                if (!clickable) referenceElement.addEventListener('mouseleave', hideHandler);
            }
            } else {
                referenceElement = triggerElements[0];
            }

            return () => {
                triggerElements.forEach((element: HTMLElement) => {
                    if (element) {
                        for (const [name, handler] of events) element.removeEventListener(name, handler);
                    }
                });
                if (referenceElement) {
                    referenceElement.addEventListener('focusout', hideHandler);
                    referenceElement.addEventListener('mouseleave', hideHandler);
                }
            };
    });
</script>

{#if !referenceElement}
    <div bind:this={contentElement} />
{/if}

{#if open && referenceElement}
    <Frame {...$$restProps} use={init} options={referenceElement} role="tooltip" tabindex={active ? -1 : undefined} on:focusin on:focusin={optional(active, showHandler)} on:focusout={optional(active, hideHandler)} on:mouseenter={optional(active && !clickable, showHandler)} on:mouseleave={optional(active && !clickable, hideHandler)}>
        <slot />
        {#if arrow}<div use:initArrow class="arrow" />{/if}
    </Frame>
{/if}

<style lang="scss">
    .arrow {
        position: absolute;
        pointer-events: none;
        display: block;
        width: 10px;
        height: 10px;
        rotate: 45deg;
        background: inherit;
        border: inherit;
    }
</style>