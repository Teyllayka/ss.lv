<script lang="ts">
  export let name;
  export let type;
  export let placeholder;
  export let errors: any[] = [];
  export let value: string | undefined | null = null;

  let e: any = null;
  $: {
    e = errors.find((x) => x.field == name);
  }
</script>

<div class="field">
  <label for={name}>{placeholder}</label>
  <input {type} {name} id={name} class:error={!!e} {value} />
  {#if e}
    <div class="error">{e?.message || ""}</div>
  {/if}
</div>
