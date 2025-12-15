<script lang="ts">
	let { size = 36 }: { size?: number } = $props();
</script>

<svg
	width={size}
	height={size}
	viewBox="0 0 115 115"
	xmlns="http://www.w3.org/2000/svg"
	class="logo-icon"
>
	<circle cx="39" cy="24" r="15" transform="rotate(180 39 24)" fill="currentColor" />
	<circle cx="76" cy="24" r="15" transform="rotate(180 76 24)" fill="currentColor" />
	<path
		d="M64 16 L64 32 C64 32 61 26 58 26 C54 26 52 32 52 32 L52 16 C52 16 54 22 58 22 C61 22 64 16 64 16 Z"
		fill="currentColor"
	/>
	<circle cx="21" cy="56" r="15" transform="rotate(-120 21 56)" fill="currentColor" />
	<circle cx="39" cy="88" r="15" transform="rotate(-120 39 88)" fill="currentColor" />
	<path
		d="M39 73 L26 81 C26 81 30 75 28 73 C27 70 20 70 20 70 L33 63 C33 63 30 68 31 71 C33 74 39 73 39 73 Z"
		fill="currentColor"
	/>
	<circle
		cx="15"
		cy="15"
		r="15"
		transform="matrix(0.5 -0.866026 -0.866026 -0.5 99 76)"
		fill="currentColor"
	/>
	<circle cx="75" cy="88" r="15" transform="rotate(-120 75 88)" fill="currentColor" />
	<path
		d="M75 73 L88 81 C88 81 84 75 86 73 C88 70 94 70 94 70 L81 63 C81 63 85 68 83 71 C82 74 75 73 75 73 Z"
		fill="currentColor"
	/>
</svg>

<style>
	.logo-icon {
		color: var(--color-primary-fg);
	}
</style>
