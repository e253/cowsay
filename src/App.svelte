<script lang="ts">
  import Cowsay from './Cowsay.svelte'
</script>

<main class="container">
  <h1>What should the Cow Say?</h1>

  <div class="row">
    <img src="/icon.png" class="logo tauri" alt="Tauri Logo" />
  </div>

  <div class="row">
    <Cowsay />
  </div>
</main>