<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	
	let { children } = $props();

</script>

<svelte:head>

	<!-- PAGE TITLE AND DESCRIPTION	-->
	<title>Geni - Platform</title>
	<meta name="description" content="Geni shows you the health your DNA is hiding." />
	<link rel="icon" href={favicon} />

</svelte:head>

{@render children()}
