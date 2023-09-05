<script lang="ts">
</script>

<div class="container">
    <div class="section">
        <div class="header">
            <h4 class="title">Members</h4>
            <p class="subtitle">Type an email of the teammate you want to invite, you can also manage groups and permissions here.</p>
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