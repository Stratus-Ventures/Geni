<script lang="ts">
    import { Button } from "$lib";
    import SunIcon from "$lib/components/icons/animated-icons/SunIcon.svelte";
    import MoonIcon from "$lib/components/icons/animated-icons/MoonIcon.svelte";

    import { toggleMode, mode } from "mode-watcher";
	let isHovered = $state(false);
 
</script>

<Button
	onclick={toggleMode}
	variant="no_style"
	class="rounded-full relative cursor-pointer"
	onmouseenter={() => isHovered = true}
	onmouseleave={() => isHovered = false}
>
	<!-- Container to stack icons -->
	<div class="relative w-fit h-fit flex items-center justify-center">
		
		<!-- SUN ICON -->
		<!--
		   VISIBILITY: Hidden if mode is dark.
		   ANIMATION: Only plays if hovered AND currently visible (light mode).
		   This ensures that if you switch modes while hovering, the animation starts fresh.
		-->
		<div class="relative transition-all duration-200
            {mode.current === 'dark' ? 'opacity-0' : 'opacity-100'}">
			<SunIcon
				size={18}
				isHovered={isHovered && mode.current !== 'dark'}
				color={isHovered ? "var(--foreground)" : "var(--muted-foreground)"}
			/>
		</div>
		
		<!-- MOON ICON -->
		<!--
		   VISIBILITY: Hidden if mode is light.
		   ANIMATION: Only plays if hovered AND currently visible (dark mode).
		-->
		<div class="absolute transition-all duration-200
			{mode.current === 'light' ? 'opacity-0' : 'opacity-100'}">
			<MoonIcon
				size={18}
				isHovered={isHovered && mode.current === 'dark'}
				color={isHovered ? "var(--foreground)" : "var(--muted-foreground)"}
			/>
		</div>
	
	</div>
	
	<span class="sr-only">Toggle theme</span>
</Button>