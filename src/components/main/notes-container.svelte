<script lang="ts">
    import { currentNotes } from '../../stores/currentnotes';
</script>

<div class="notes-container">
    {#each $currentNotes as note}
        <section>
            {note.created_on}
        </section>
    {/each}
</div>

<style>
    div {
        overflow-y: scroll;
    }
</style>
