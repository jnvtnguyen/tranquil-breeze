<script lang="ts">
	import type { HTMLInputTypeAttribute } from "svelte/elements";
    import { createEventDispatcher, tick, onMount } from "svelte";
    import { slide } from "svelte/transition";
    //@ts-ignore
    import AlertIcon from "~icons/mdi/alert";
    //@ts-ignore
    import CheckIcon from "~icons/mdi/check-circle";
    //@ts-ignore
    import { v4 as uuid } from "uuid";

    export let value: string | null = null;
    export let label: string | null = null;
    export let error: string | null = null;
    export let dirty: boolean | null = null;
    export let valid: boolean | null = null;
    export let autofocus: boolean = false;
    export let required: boolean = false;
    export let type: HTMLInputTypeAttribute = "text";
    export let name: string;
    export let id: string = uuid();

    let input: HTMLInputElement;
    let focused: boolean = false;
    let autocomplete: string = "off";

    const dispatch = createEventDispatcher();

    const typeAction = (node: HTMLInputElement) => {
        node.type = type;
    }

    const handleFocus = () => {
        focused = true;
        dispatch("focus");
    }

    const handleBlur = () => {
        focused = false;
        dispatch("blur");
    }

    onMount(async () => {
        await tick();
        if(autofocus) {
            input.focus();
        }
        autocomplete = "on";
    });
</script>

<div class="wrapper">
    {#if label}
        <label class="label" for={id}>
            {label}
            {#if required}
                <span class="required">*</span>
            {/if}
        </label>
    {/if}
    <div class="input-wrapper" class:error class:focused>
        <input 
            {...$$restProps}
            class="input"
            id={id}
            name={name}
            {autocomplete}
            bind:value 
            bind:this={input}
            on:focus={handleFocus}
            on:blur={handleBlur}
            use:typeAction
        />
        {#if error}
            <span class="suffix-icon" class:error>
                <AlertIcon />
            </span>
        {:else if dirty}
            <span class="suffix-icon" class:valid>
                <CheckIcon />
            </span>
        {/if}
    </div>
    {#if error}
        <p class="error-message" transition:slide>
            {error}
        </p>
    {/if}
</div>

<style lang="scss">
    @use "$lib/css/variables" as *;
    .wrapper {
        position: relative;
        margin-bottom: $spacing-2;
    } 

    .label { 
        position: relative; 
        left: 0;
        right: 0;
        top: 0;
        padding: 0;
        text-align: left;
        font-size: 14px;
        cursor: text;
    }

    .input-wrapper {
        display: flex;
        align-items: center;
        margin-top: $spacing-1; 
        height: 48px;
        width: 100%;
        margin-bottom: 0;
        border: 1px solid $color-border;
        border-radius: $spacing-half;
        background-color: #efefef3d;
        cursor: text;
        transition: all 0.2s;

        &:hover {
            border-color: $color-border-hover;
        }

        &.focused:not(.error) {
            border-color: $color-border-focus;
        }

        &.error {
            border-color: $color-danger;
        }
    }

    .input {
        flex: 1;
        font-size: 14px;
        border: 0;
        border-radius: $spacing-half;
        outline: none;
        background: transparent;
        padding: $spacing-2;
        height: 100%;
        width: 100%;
        &::placeholder {
            color: $color-faint-text;
        }
    }

    .required {
        font-size: 16px;
        color: $color-danger;
    }

    .error-message {
        height: auto;
        margin-top: $spacing-1;
        margin-bottom: 0;
        font-size: 14px;
        color: $color-danger;
    }

    .suffix-icon {
        display: inline-flex;
        position: absolute;
        align-items: center;
        right: 0;
        margin-right: $spacing-2;
        &.error {
            color: $color-danger;
        }

        &.valid {
            color: $color-success;
        }
    }
</style>