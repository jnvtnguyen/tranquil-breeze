<script lang="ts">
	import type { HTMLInputTypeAttribute } from "svelte/elements";

    export let value: string | null = null;
    export let label: string | null = null;
    export let type: HTMLInputTypeAttribute = "text";
    export let name: string;

    let input: HTMLInputElement;
    let focused: boolean = false;

    const typeAction = (node: HTMLInputElement) => {
        node.type = type;
    }

    const handleFocus = () => {
        focused = true;
    }

    const handleBlur = () => {
        focused = false;
    }
</script>

<div class="wrapper">
    {#if label}
        <label class="label" for={name}>{label}</label>
    {/if}
    <input 
        {...$$restProps}
        class="input"
        class:focused
        name={name}
        bind:value 
        bind:this={input}
        on:focus={handleFocus}
        on:blur={handleBlur}
        use:typeAction
    />
</div>

<style lang="scss">
    @use "$lib/css/spacing" as *;

    .wrapper {
        position: relative;
        margin-bottom: $spacing-1;
    } 

    .label {
        position: relative;
        left: 0;
        right: 0;
        top: 0;
        padding: 0;
        text-align: left;
        font-size: 12px;
        color: #625F63;
    }

    .input {
        height: 48px;
        width: 100%;
        margin-top: $spacing-1;
        margin-bottom: 0;
        padding: $spacing-2;
        border: 1px solid #72727E;
        font-size: 16px;
    }
</style>