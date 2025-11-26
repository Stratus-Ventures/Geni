<script lang="ts">
    // "Dumb" Component - It just does what the parent tells it
    let {
        color = 'currentColor',
        size = 24,
        strokeWidth = 2,
        isHovered = false, // Controlled by parent
        class: className = ''
    } = $props();

    const sunRays = [
        'M12 2v2', 'm19.07 4.93-1.41 1.41', 'M20 12h2', 'm17.66 17.66 1.41 1.41',
        'M12 20v2', 'm6.34 17.66-1.41 1.41', 'M2 12h2', 'm4.93 4.93 1.41 1.41'
    ];
</script>

<div class={className} aria-label="sun" role="img">
	<svg
		xmlns="http://www.w3.org/2000/svg"
		width={size}
		height={size}
		viewBox="0 0 24 24"
		fill="none"
		stroke={color}
		stroke-width={strokeWidth}
		stroke-linecap="round"
		stroke-linejoin="round"
		class="sun-icon"
		class:animate={isHovered}
	>
		<circle cx="12" cy="12" r="4" />
		{#each sunRays as d, index}
			<!-- We add a tiny delay based on index for the wave effect -->
			<path {d} class="sun-ray" style="--index: {index}" />
		{/each}
	</svg>
</div>

<style>
    .sun-icon { overflow: visible; }

    .sun-ray {
        opacity: 1;
        transition: opacity 0.3s;
    }

    /* Only animate when parent says isHovered */
    .sun-icon.animate .sun-ray {
        opacity: 0;
        animation: fadeIn 0.4s cubic-bezier(0.4, 0, 0.2, 1) forwards;
        animation-delay: calc(var(--index) * 0.05s);
    }

    @keyframes fadeIn {
        from { opacity: 0; }
        to { opacity: 1; }
    }
</style>