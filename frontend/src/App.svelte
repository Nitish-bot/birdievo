<script lang="ts">
  import init, * as sim from 'simulation-wasm'
  import { onMount } from 'svelte'  

  onMount(async () => {
    await init()

    alert(sim.whos_that_dog())
  })
</script>

<main>
  <p class="read-the-docs">
    Click on the Vite and Svelte logos to learn more
  </p>
</main>
