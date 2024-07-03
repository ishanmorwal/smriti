<!-- YOU CAN DELETE EVERYTHING IN THIS PAGE -->
<script lang="ts">
import { appWindow } from '@tauri-apps/api/window';
async function closeApp() {
    try {
      await appWindow.close();
    } catch (error) {
      console.error("Failed to close the app:", error);
    }
  }
</script>
<div class="container h-full mx-auto flex flex-col p-[256px] gap-5 justify-center items-center">
	<header class='text-8xl'>Smriti</header>
	<div class="container mx-auto flex flex-col text-xl items-center justify-center">
		<span class="btn text-xl"><a href='/write'>Write a note</a></span>
		<span class="btn text-xl"><a href='/open'>Open a note</a></span>
		<button class="btn text-xl" on:click={closeApp()}>Exit</button>
	</div>
</div>
