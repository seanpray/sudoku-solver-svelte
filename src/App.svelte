<script lang="ts">
    import { onMount } from "svelte";
    import wasm from '../solver-wasm/Cargo.toml';
    const default_val = new Array(0,0,8,4,3,1,2,6,9,6,0,9,7,2,0,8,3,4,0,3,0,0,0,6,1,0,5,0,0,0,0,0,0,9,8,0,0,0,6,9,0,2,0,0,0,4,0,0,3,8,0,5,0,6,8,0,0,5,4,0,6,9,0,0,0,3,0,0,0,7,0,2,1,6,5,2,0,9,3,4,0);
    let solverwasm;
    onMount(async () => {
        solverwasm = await wasm();
    });
    let cells = JSON.parse(JSON.stringify(default_val));
    let inputData;
    let time = 0;
    let message = "";
</script>

<main>
    <div class="sudoku-grid">
        {#each cells as item, index}
            <div class="index sudoku-grid-item">
                <!-- {#if index > 9 && index % 27 == 0} -->
                <!-- ______ -->
                <!-- {/if} -->
                <!-- <br /> -->
                <!-- {#if index % 9 != 0 && index % 3 == 0} -->
                <!--     | -->
                <!-- {/if} -->
                <input type="number" min="0" max="9" value={item} />
            </div>
        {/each}
    </div>
    <div class="message">
        {message}
    </div>
    <div>
        Complete in {time} ms
    </div>
    <button on:click={() => {
        const start = Date.now();
        const result = solverwasm.solve(cells);
        if (result.includes(0)) {
            message = "testset";
        }
        if (result && result.length > 0) {
            cells = result;
        } else {
            message = "No solution";
        }
        time = Date.now() - start;
    }}>
        Solve
    </button>
    <button on:click={() => cells = new Array(81).fill(0)}>
        Clear
    </button>
    <button on:click={() => cells = JSON.parse(JSON.stringify(default_val))}>
        Default
    </button>
    <div class="inputBox">
        <input bind:value={inputData} on:input={(_) => {
            if (inputData && inputData.length == 81) {
                cells = inputData.split("").map((i) => parseInt(i));
            }
        }}/>
    </div>
</main>

<style>
    .message {
        color: #ff0000;
    }
    .inputBox {
        width: 50%;
    }
    .sudoku-grid {
        margin: auto;
        display: grid;
        gap: 1px;
        grid-template-columns: repeat(9, minmax(0, 1fr));
    }
    .sudoku-grid-item {
        font-size: 20px;
          text-align: center;
    }
    input[type=number]::-webkit-inner-spin-button,
    input[type=number]::-webkit-outer-spin-button {
      -webkit-appearance: none;
      margin: 0;
    }
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
