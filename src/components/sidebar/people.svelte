<script lang="ts">
    import { invoke } from '@tauri-apps/api';

    import { onMount } from 'svelte';
    import { fly, fade } from 'svelte/transition';
    import { people } from '../../stores/people';
    import { selectedPerson } from '../../stores/selected_person';

    onMount(async () => {
        $people = await invoke('get_persons');
    });
</script>

<main in:fade>
    <h2>People</h2>
    <section>
        {#each $people as person, i}
            <div in:fly={{ x: -50 }} on:click={() => ($selectedPerson = person)}>
                <span in:fly={{ x: -50, delay: 50 * i }}>{person.name}</span>
            </div>
        {/each}
    </section>
</main>

<style lang="scss">
    main {
        padding: 0.5rem;
        margin: 0.5rem;

        border-radius: 8px;

        background-color: #d6ccc2;
    }

    h2 {
        margin-top: 0.15rem;
        margin-bottom: 0.75rem;
    }

    span {
        font-size: 0.85rem;
    }
</style>
