<script>
  import cowsay from 'cowsay2'

  let name = "";
  let greetMsg = ""

  async function greet() {
    greetMsg = cowsay.say({ text: name })
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter any text..." bind:value={name} />
    <button type="submit">Say!</button>
  </form>
  <p>{greetMsg}</p>
</div>