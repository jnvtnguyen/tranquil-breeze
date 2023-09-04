<script lang="ts">
	import type { Workspace } from "$lib/types";

    export let workspace: Workspace;
</script>

<div class="container">
    <div class="section">
        <div class="header">
            <h4 class="title">Members</h4>
            <p class="subtitle">Type an email of the teammate you want to invite, manager user permissions, remove users.</p>
        </div>
    </div>
</div>

<style lang="scss">
    @use "$lib/css/variables" as *;

    .section {
        margin-left: $spacing-10;
        margin-top: $spacing-5;

        .header {
            max-width: 480px;

            .title {
                margin: 0;
            }

            .subtitle {
                color: $color-faint-text;
            }
        }
    }
</style>