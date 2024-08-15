<script lang="ts">
  import type { PageData } from "./$houdini";
  export let data: PageData;
  $: ({ me } = data);
</script>

{JSON.stringify($me)}
<h1>Welcome to SvelteKit</h1>
<p>
  Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
</p>
