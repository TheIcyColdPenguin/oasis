<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api';

    let text = 'hi';

    onMount(() => {
        setTimeout(async () => {
            text = JSON.stringify(await invoke('hello'));
        }, 1000);
    });
</script>

<main>{text}</main>

<style>
    main {
        width: 100vw;
        height: 100vh;

        margin: 0;
        padding: 0;

        display: grid;
        place-items: center;

        text-align: center;
    }
</style>
