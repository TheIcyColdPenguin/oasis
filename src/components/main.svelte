<script lang="ts">
    import People from './sidebar/people.svelte';
</script>

<main>
    <aside>
        <People />
    </aside>
</main>

<style>
    main {
        width: 100vw;
        height: 100vh;

        margin: 0;
        padding: 0;

        display: grid;
        grid-template-columns: minmax(200px, 2fr) 8fr;
    }

    aside {
        border-right: 3px solid #ffffff33;
    }
</style>
