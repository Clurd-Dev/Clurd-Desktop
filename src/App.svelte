<script lang="ts">
	import { parse_config } from './ts/io';
	import { onMount } from 'svelte';
	import { Router, Link, Route } from "svelte-navigator";
	import File from './File.svelte';
	import Init from './Init.svelte';
	let url: string, nameserver: string;
	onMount(async ()=>{
		let config: any = await parse_config();
		url = config.url;
		nameserver = config.name;
	});
  </script>
<Router>
	<main>
	  <Route path="file">
		<File url={url}/>
	  </Route>
  
  
	  <Route>
		  <Init url={url}/>
		<Link to="file">{nameserver} - {url}</Link>	  
	</Route>
	</main>
</Router>
  