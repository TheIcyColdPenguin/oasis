<script lang="ts">
    import { onMount } from 'svelte';
    import { fade } from 'svelte/transition';

    import Facade from './components/facade.svelte';
    import Main from './components/main.svelte';

    let showingIntro = true;

    onMount(() => {
        setTimeout(() => {
            showingIntro = false;
        }, 1000);
    });
</script>

{#if showingIntro}
    <main out:fade={{ duration: 100 }}>
        <Facade />
    </main>
{:else}
    <main in:fade={{ delay: 150, duration: 100 }}>
        <Main />
    </main>
{/if}

<style>
    main {
        width: 100vw;
        height: 100vh;

        margin: 0;
        padding: 0;

        display: grid;
        place-items: center;
    }
</style>
