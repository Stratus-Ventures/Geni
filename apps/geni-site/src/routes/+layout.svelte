<script lang="ts">
	import '../app.css';

	let { children } = $props();
</script>

<svelte:head>
	<title>Geni - Your DNA, Your Health Insights</title>
	<meta
		name="description"
		content="Transform your raw DNA data into actionable health insights. Educational genetic analysis for 23andMe and Ancestry files."
	/>
</svelte:head>

{@render children()}
