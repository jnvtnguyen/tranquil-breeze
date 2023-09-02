<script lang="ts">
	import type { ComponentProps } from "svelte";
    import Popper from "./Popper.svelte";

    interface $$Props extends ComponentProps<Popper> {
        open?: boolean;
    }

    export let open: boolean = false;

    $: {
        $$restProps.arrow = $$restProps.arrow ?? false;
        $$restProps.trigger = $$restProps.trigger ?? "click";
        $$restProps.placement = $$restProps.placement ?? "bottom";
    }
</script>

<Popper {...$$restProps} active on:show bind:open>
    <slot /> 
</Popper>