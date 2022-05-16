<script lang="ts">
import { add } from './ts/add_server';
import { invoke } from '@tauri-apps/api/tauri';
export let url: string;
</script>
<button on:click={add}>Add new server</button>
<h1>Available servers</h1>