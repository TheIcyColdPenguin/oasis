<script lang="ts">
    import { fade } from 'svelte/transition';

    import { selectedPerson } from '../../stores/selected_person';
    import NotesContainer from './notes-container.svelte';
</script>

<main>
    {#if $selectedPerson}
        {#key $selectedPerson?.id}
            <div in:fade={{ delay: 100, duration: 100 }} out:fade={{ duration: 50 }}>
                <h1>{$selectedPerson.name}</h1>
                <NotesContainer />
            </div>
        {/key}
    {/if}
</main>

<style>
    main,
    main > div {
        width: 100%;
        height: 100%;

        margin: 0;
        padding: 1rem;

        display: grid;
        grid-template-rows: 1fr 8fr;
    }

    h1 {
        margin-top: 0.25rem;
    }
</style>
