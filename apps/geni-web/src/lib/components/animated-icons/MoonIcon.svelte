<script lang="ts">
    let {
        color = 'currentColor',
        size = 24,
        strokeWidth = 2,
        isHovered = false, // Controlled by parent
        class: className = ''
    } = $props();
</script>

<div class={className} aria-label="moon" role="img">
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
		class="moon-icon"
		class:animate={isHovered}
	>
		<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />
	</svg>
</div>

<style>
    .moon-icon { overflow: visible; }

    /* The animation runs once. When isHovered becomes false, the class is removed, resetting it for next time. */
    .moon-icon.animate {
        animation: moonRotation 1.2s ease-in-out;
    }

    @keyframes moonRotation {
        0% { transform: rotate(0deg); }
        20% { transform: rotate(-10deg); }
        40% { transform: rotate(10deg); }
        60% { transform: rotate(-5deg); }
        80% { transform: rotate(5deg); }
        100% { transform: rotate(0deg); }
    }
</style>