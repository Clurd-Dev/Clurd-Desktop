<script lang="ts">
	import { parse_config } from './ts/io';
	import { onMount } from 'svelte';
	import { Router, Link, Route } from "svelte-navigator";
	import File from './File.svelte';
	import Init from './Init.svelte';
	import Upload from './Upload.svelte';
	let url: string, nameserver: string;
	onMount(async ()=>{
		let config: any = await parse_config();
		url = config.url;
		nameserver = config.name;
	});
  </script>
<Router>
	<main>
		<nav class="uk-navbar-container" uk-navbar>
			<div class="uk-navbar-left">
		
				<ul class="uk-navbar-nav">
					<li>
						<Link to="/">Clurd</Link>
					</li>
					<li>
						<Link to="upload">Upload</Link>
					</li>
				</ul>
		
			</div>
		</nav>
	  <Route path="file">
		<File url={url}/>
	  </Route>
  
	  <Route path="upload">
		<Upload url={url}></Upload>
	  </Route>
  
	  <Route>
		  <Init url={url}/>
		<Link to="file">{nameserver} - {url}</Link>	  
	</Route>
	</main>
</Router>
  