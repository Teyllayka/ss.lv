<script lang="ts">
export let data;
</script>

{#if data && data.errors && data.errors.length > 0}
  errors
  <a href="/">wada</a>
{:else if data && data.data && data.data.verifyEmail == "Email verified"}
  success

  <a href="/">wada</a>
{:else}
  loading
{/if}
