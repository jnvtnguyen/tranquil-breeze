<script lang="ts">
    import { LINKS } from "$lib/links";
	import type { Workspace } from "$lib/types";
    import { page, navigating } from "$app/stores";
    //@ts-ignore
    import ForwardIcon from "~icons/carbon/chevron-right";
    //@ts-ignore
    import BackwardIcon from "~icons/carbon/chevron-left";

    export let workspace: Workspace;

    $: root = `/workspaces/${workspace.slug}`;
    $: current = $page.url.pathname.replace(root, "");
    $: backwards = LINKS.findIndex((link) => link.href === current) - 1;
    $: crumbs = LINKS.slice(0, backwards + 2);
</script>

<ol class="breadcrumbs">
    <li class="controls">
        {#if !$navigating}
            {#if backwards !== -1 && LINKS[backwards]}
                <a class="icon" href={root + LINKS[backwards].href} class:able={backwards + 1}>
                    <BackwardIcon />
                </a>
            {:else}
                <div class="icon">
                    <BackwardIcon />
                </div>
            {/if}
            <div class="icon">
                <ForwardIcon />
            </div>
        {:else}
            <div class="icon">
                <BackwardIcon />
            </div>
            <div class="icon">
                <ForwardIcon />
            </div>
        {/if}
        
    </li>
    {#each crumbs as crumb}
        {#if crumb.href === ""}
            <li>
                <a href={root} class="workspace">
                    <img class="image" src="https://picsum.photos/200/200" />
                </a>
            </li>
        {:else}
            <li>
                <a href={root + crumb.href} class="crumb">
                    {#if crumb.icon}
                        <div class="icon">
                            <svelte:component this={crumb.icon} />
                        </div>
                    {/if}
                    <span class="title">{crumb.title}</span>
                </a>
            </li>
        {/if}
        <li class="separator">/</li>
    {/each}
</ol>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .breadcrumbs {
        display: flex;
        align-items: center;
        background: $color-sidebar;
        margin-left: $spacing-2;
        min-height: 40px;
        border-radius: 30px;
        padding: $spacing-half;
        list-style: none;
        border: 1px solid $color-border;
    }

    .controls {
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 60px;
        border-radius: 30px;
        background: $color-sidebar;
        padding: $spacing-1;
        color: $color-faint-text;
        box-shadow: rgba(60, 64, 67, 0.3) 0px 1px 2px 0px, rgba(60, 64, 67, 0.15) 0px 1px 3px 1px;

        .icon {
            color: $color-grey;
            display: flex;
            align-items: center;
            cursor: pointer;

            &.able {
                color: $color-text;
            }

            &.activated {
                color: $color-inverse-text;
            }
        }
    }

    .workspace {
        display: flex; 
        align-items: center;

        .image {
            width: 25px;
            height: 25px;
            border-radius: 25%;
            margin-left: $spacing-2;
            box-shadow: rgba(60, 64, 67, 0.3) 0px 1px 2px 0px, rgba(60, 64, 67, 0.15) 0px 1px 3px 1px;
        }
    }

    .crumb {
        display: flex;
        align-items: center;
        color: $color-faint-text;
        text-decoration: none;
        transition: all .2s;

        .title {
            margin-left: $spacing-1;
        }

        .icon {
            display: flex;
            align-items: center;
            justify-content: center;
        }
    }

    .separator {
        color: $color-faint-text;
        margin-left: $spacing-2;
        margin-right: $spacing-2;
    }
</style>