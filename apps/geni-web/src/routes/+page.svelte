<script lang="ts">
	import { AuthForm } from "$lib/auth";
</script>

<main class="grad-bg">
	<section>
		<AuthForm />
	</section>
</main>

<style>
	
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		width: 100%;
		height: 100%;
	}
	
	.grad-bg {
        background: var(--gradient-main), var(--color-primary-bg);
	}
</style>