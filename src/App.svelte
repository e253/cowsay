<script lang="ts">
  import { onUpdaterEvent, checkUpdate, installUpdate } from '@tauri-apps/api/updater'
  import { getVersion } from '@tauri-apps/api/app'
  import { onMount } from 'svelte'

  let updateState: 'noUpdates' | 'updateAvailable' | 'updateDownloading' | 'restarting' = "noUpdates";
  let version: string
  let newVersion: string | undefined
  onMount(async () => {
    version = await getVersion()

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
        newVersion = manifest?.version
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
    <p>
      No Updates Available (v{version})
    </p>
  {:else if updateState === "updateAvailable"}
    <p>Update Available! (v{newVersion})</p>
  {:else if updateState === "updateDownloading"}
    <p>Version {newVersion} is Beind Downloaded</p>
  {:else}
    <p>Please Restart app to upgrade to version {newVersion}</p>
  {/if}
</div>
