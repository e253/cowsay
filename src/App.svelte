<script lang="ts">
  let name = ""
  let greetMsg = ""
</script>

<main class="container" >
  <div class="cow">
    <div class="row">
      <img src="/icon.png" class="logo tauri" alt="Tauri Logo" />
    </div>
    {#if greetMsg}
      <div class="cowsay">
        ^  <br>
        |  <br>
        |
        <p class="cowsay-msg">{greetMsg}</p>
      </div>
    {/if}
  </div>

  <div class="row">
    <form class="row" on:submit|preventDefault={async () => {greetMsg = name}}>
      <input id="greet-input" placeholder="Enter any text..." bind:value={name} />
      <button type="submit">Say!</button>
    </form>
  </div>
</main>