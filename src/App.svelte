<script lang="ts">
  import { onUpdaterEvent, checkUpdate, installUpdate } from '@tauri-apps/api/updater'
  import { relaunch } from '@tauri-apps/api/process'
  import { onMount } from 'svelte'

  let updateState: 'noUpdates' | 'updateAvailable' | 'updateDownloading' | 'restarting' = "noUpdates";

  onMount(async () => {
    const unlisten = await onUpdaterEvent(({ error, status }) => {
      console.log('Updater event', error, status)
    })

    try {
      const { shouldUpdate, manifest } = await checkUpdate()

      if (shouldUpdate) {
        updateState = 'updateAvailable'
        // You could show a dialog asking the user if they want to install the update here.
        console.log(
          `Installing update ${manifest?.version}, ${manifest?.date}, ${manifest?.body}`
        )
        updateState = 'updateDownloading';
        // Install the update. This will also restart the app on Windows!
        await installUpdate()

        updateState = 'restarting'
        // On macOS and Linux you will need to restart the app manually.
        // You could use this step to display another confirmation dialog.
        // await relaunch()
      }
    } catch (error) {
      console.error(error)
    }

    unlisten();
  })

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

<div class="container update-status">
  {#if updateState === "noUpdates"}
    <p>No Updates Available</p>
  {:else if updateState === "updateAvailable"}
    <p>Update Available!</p>
  {:else if updateState === "updateDownloading"}
    <p>Update is Beind Downloaded</p>
  {:else}
    <p>Restarting Application</p>
  {/if}
</div>
