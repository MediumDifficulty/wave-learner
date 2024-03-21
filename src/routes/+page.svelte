<script lang="ts">
    import WaveDrawer from "$lib/WaveDrawer.svelte";
    import { WAVE_RES } from "$lib/consts";
    import { waveData } from "$lib/stores";
    import * as wasm from "$lib/wasm/trainer"
    import { onMount } from "svelte";

    let bestLine = new Float32Array(WAVE_RES)
    let bestFormula = ""

    let stepCount = 1

    onMount(() => {
        wasm.init()
        console.log($waveData)
        // wasm.greet()
    })

    function step() {
        for (let i = 0; i < stepCount; i++)
            wasm.step_training()

        bestLine = wasm.get_best_output()
        bestFormula = wasm.get_best_formula()
    }
</script>

<main>    
    <div class="drawer-container">
        <WaveDrawer bestLine={bestLine} />
        <div>{bestFormula}</div>
    </div>
    <button on:click={() => wasm.init_training($waveData)}>Start Training</button>
    <input type="number" bind:value={stepCount} />
    <button on:click={step}>Step Training</button>
</main>

<style>
    :global(body) {
        margin: 0;
        font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    }

    .drawer-container {
        width: 70%;
        margin-left: auto;
        margin-right: auto;
        margin-top: 5rem;
        aspect-ratio: 4/3;
    }

    main {
        margin-left: auto;
        margin-right: auto;
        width: 75%;
    }
</style>