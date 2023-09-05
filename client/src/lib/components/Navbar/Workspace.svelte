<script lang="ts">
    import { navigating, page } from "$app/stores";
    //@ts-ignore
    import MagnifyIcon from "~icons/mdi/magnify";
    //@ts-ignore
    import MenuDownIcon from "~icons/mdi/menu-down";
    //@ts-ignore
    import MenuUpIcon from "~icons/mdi/menu-up";
	import WorkspaceList from "./WorkspaceList.svelte";
	import WorkspaceModal from "./WorkspaceModal.svelte";
    import { Dropdown } from "$lib/components";
	import { fly } from "svelte/transition";

    let workspaceOpen: boolean = false;
    let createWorkspaceOpen: boolean = false;

    const handleModalOpen = () => {
        createWorkspaceOpen = true;
        workspaceOpen = false;
    }
</script>

<div>
    <div class="workspace">
        <div class="content">
            <img class="image" src="https://picsum.photos/200/200" />
            <span class="name">{$page.data.workspace.name}</span>
        </div>
        <div class="icon">
            {#if !$navigating}
                {#if workspaceOpen}
                    <MenuUpIcon />
                {:else}
                    <MenuDownIcon />
                {/if}
            {:else}
                <MenuDownIcon />
            {/if}
        </div>
    </div>

    <Dropdown bind:open={workspaceOpen} offset={4} placement="bottom-start" transition={fly} params={{ duration: 300, y: 5 }}>
        <div class="dropdown">
            <div class="search">
                <div class="icon">
                    <MagnifyIcon />
                </div>
                <input autocomplete="off" name="search" class="input" placeholder="Search for a workspace" />
            </div>

            <WorkspaceList />

            <div class="workspaces-actions">
                <span class="text" on:click={handleModalOpen}>Create New Workspace</span>
            </div>

        </div>
    </Dropdown>
    <WorkspaceModal bind:open={createWorkspaceOpen} />
</div>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .workspace {
        display: flex;
        align-items: center;
        justify-content: space-between;
        background: $color-sidebar;
        padding: $spacing-1;
        border-radius: $spacing-1;
        cursor: pointer;
        width: 208px;
        color: $color-text;
        user-select: none;
        box-shadow: rgba(60, 64, 67, 0.3) 0px 1px 2px 0px, rgba(60, 64, 67, 0.15) 0px 2px 6px 2px;

        .content {
            display: flex;
            align-items: center;

            .image {
                width: 30px;
                height: 30px;
                border-radius: 25%;
                box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;
            }

            .name {
                display: block;
                font-size: 15px;
                padding-left: $spacing-1-half;
                text-overflow: ellipsis;
                white-space: nowrap;
                overflow: hidden;
                width: 125px;
            }
        }

        .icon {
            display: flex;
            align-items: center;
            padding-left: $spacing-1-half;
            font-size: 18px;
        }
    }

    .dropdown {
        min-width: 360px;

        .loading {
            display: flex;
            align-items: center;
            justify-content: center;
            padding: $spacing-2;
        }

        .search {
            display: flex;
            align-items: center;
            color: $color-faint-text;

            .input {
                flex: 1;
                border: 0;
                width: 100%;
                height: 100%;
                padding: $spacing-2;
                padding-left: $spacing-5;
                font-size: 14px;
                outline: none;
                background: transparent;
            }

            .icon {
                position: absolute;
                display: flex;
                align-items: center;
                font-size: 18px;
                padding-left: $spacing-1;
            }
        }

        .workspaces-actions {
            padding: $spacing-2;
            font-size: 14px;
            
            .text {
                cursor: pointer;
            }
        }
    }
</style>