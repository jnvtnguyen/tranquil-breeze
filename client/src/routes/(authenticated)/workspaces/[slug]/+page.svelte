<script lang="ts">
    import type { PageData } from "./$types";
    import { PageMeta } from "$lib/components/";

    export let data: PageData;
</script>

<PageMeta title={data.workspace.name} />