<script lang="ts">
    import { onMount } from 'svelte'
    import { check } from '@tauri-apps/plugin-updater'

    const checkUpdate = async () => {
        const update = await check();
        console.log(update)
        if (update?.meta?.available) {
            console.log("Available!")
        }

        setTimeout(checkUpdate, 300)
    }

    onMount(checkUpdate)
</script>

