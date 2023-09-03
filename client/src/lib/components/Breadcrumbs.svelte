<script lang="ts">
	import type { Workspace } from "$lib/types";
    //@ts-ignore
    import ForwardIcon from "~icons/carbon/chevron-right";
    //@ts-ignore
    import BackwardIcon from "~icons/carbon/chevron-left";

    export let workspace: Workspace;
</script>

<ol class="breadcrumbs">
    <li class="controls">
        <a class="icon" href="">
            <BackwardIcon />
        </a>
        <a class="icon" href="">
            <ForwardIcon />
        </a>
    </li>
    <li>
        <a href={`/workspaces/${workspace.slug}`} class="workspace">
            <img class="image" src="https://picsum.photos/200/200" />
        </a>
    </li>
    <li class="separator">/</li>
</ol>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .breadcrumbs {
        display: flex;
        align-items: center;
        background: $color-light-grey;
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
            color: $color-faint-text;
            display: flex;
            align-items: center;
            cursor: pointer;

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

    .separator {
        color: $color-faint-text;
        margin-left: $spacing-2;
        margin-right: $spacing-2;
    }
</style>