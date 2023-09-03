<script lang="ts" context="module">
    export type ModalPlacementType = 'top-left' | 'top-center' | 'top-right' | 'center-left' | 'center' | 'center-right' | 'bottom-left' | 'bottom-center' | 'bottom-right'; 
</script>

<script lang="ts">
	import type { SizeType } from "./types";
	import { createEventDispatcher, type ComponentProps } from "svelte";
    //@ts-ignore
    import CloseIcon from "~icons/mdi/close";
    import focusTrap from "./util/FocusTrap";
    import Frame from "./Frame.svelte";

    interface $$Props extends ComponentProps<Frame> {
        open?: boolean;
        title?: string;
        placement?: ModalPlacementType;
        autoclose?: boolean;
        outsideclose?: boolean;
        dismissible?: boolean;
        size?: SizeType;
    }

    export let open: boolean = false;
    export let title: string = '';
    export let placement: ModalPlacementType = "center";
    export let autoclose: boolean = false;
    export let outsideclose: boolean = false;
    export let dismissible: boolean = true;
    export let size: SizeType = "md";

    const getPlacementStyles = () => {
        switch(placement) {
            case "top-left":
                return "justify-content: flex-start; align-items: flex-start;";
            case "top-center":
                return "justify-content: center; align-items: flex-start;";
            case "top-right":
                return "justify-content: flex-end; align-items: flex-start;";
            case "center-left":
                return "justify-content: flex-start; align-items: center;";
            case "center":
                return "justify-content: center; align-items: center;"; 
            case "center-right":
                return "justify-content: flex-end; align-items: center;";
            case "bottom-left":
                return "justify-content: flex-start; align-items: flex-end;";
            case "bottom-center":
                return "justify-content: center; align-items: flex-end;";
            case "bottom-right":
                return "justify-content: flex-end; align-items: flex-end;";
        }
    }

    const getSizeStyles = () => {
        switch(size) {
            case "xs":
            case "sm":
                return "max-width: 32rem;"
            case "md":
                return "max-width: 42rem;"
            case "lg":
            case "xl":
        }
    }

    const dispatch = createEventDispatcher();
    $: dispatch(open ? "open" : "close");

    const isScrollable = (e: HTMLElement): boolean[] => [e.scrollWidth > e.clientWidth && ['scroll', 'auto'].indexOf(getComputedStyle(e).overflowX) >= 0, e.scrollHeight > e.clientHeight && ['scroll', 'auto'].indexOf(getComputedStyle(e).overflowY) >= 0];

    const prepareFocus = (node: HTMLElement) => {
        const walker = document.createTreeWalker(node, NodeFilter.SHOW_ELEMENT);
        let n: Node | null;
        while((n = walker.nextNode())) {
            if(n instanceof HTMLElement) {
                const e = n as HTMLElement;
                const [x, y] = isScrollable(e);
                if(x || y) e.tabIndex = 0;
            }
        }
        node.focus();
    }

    const hide = (e: Event) => {
        e.preventDefault();
        open = false;
    }

    const onAutoClose = (e: MouseEvent) => {
        const target: Element = e.target as Element;
        if(autoclose && target?.tagName == "BUTTON") hide(e);
        if(outsideclose && target === e.currentTarget) hide(e);
    };

    const handleKeys = (e: KeyboardEvent) => {
        if(e.key === "Escape" && dismissible) return hide(e);
    }
</script>

{#if open}
    <div class="backdrop" />

    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <div on:keydown={handleKeys} on:wheel|preventDefault|nonpassive use:prepareFocus use:focusTrap on:click={onAutoClose} class="modal" tabindex="-1" aria-modal="true" role="dialog" style={getPlacementStyles()}>
        <div class="container" style={getSizeStyles()}>
            <Frame {...$$restProps}>
                {#if $$slots.header || title}
                    <div class="header">
                        <slot name="header">
                            <h3 class="title">
                                {title}
                            </h3>
                        </slot>
                        {#if dismissible}
                            <button class="close" on:click={hide}>
                                <CloseIcon  />
                            </button>
                        {/if}
                    </div>
                {/if}
                <div class="body">
                    <slot />
                </div>
                {#if $$slots.footer}
                    <div class="footer">
                        <slot name="footer" />
                    </div>
                {/if}
            </Frame>
        </div>
    </div>
{/if}

<style lang="scss">
    @use "$lib/css/variables" as *;

    .modal {
        height: 100%;
        width: 100%;
        display: flex;
        z-index: 50;
        position: fixed;
        inset: 0px;
    }

    .backdrop {
        position: fixed;
        top: 0;
        left: 0;
        opacity: 0.5;
        width: 100%;
        height: 100%;
        background-color: $color-backdrop;
        z-index: 40;
    }

    .container {
        position: relative;
        display: flex;
        flex-direction: column;
        width: 100%;
        max-height: 100%;
        margin-right: auto;
        margin-left: auto;
    }

    .header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        color: $color-text;
        padding: $spacing-2;
        padding-top: $spacing-half;
        padding-bottom: $spacing-half;
        border-bottom: 1px solid $color-border;
        
        .close {
            display: flex;
            align-items: center;
            justify-content: center;
            border: 0;
            background: transparent;
            padding: 0;
            margin: 0;
            cursor: pointer;
            color: $color-faint-text;
            font-size: 18px;
        }
    }

    .body {
        padding: $spacing-2;
        border-bottom: 1px solid $color-border;
    }

    .footer {
        display: flex;
        align-items: center;
        justify-content: flex-start;
        padding: $spacing-2;
    }
</style>