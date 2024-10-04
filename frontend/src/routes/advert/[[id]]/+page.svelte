<script lang="ts">
import type { PageData } from "./$houdini";
export let data: PageData;
$: ({ Advert } = data);
</script>

{JSON.stringify($Advert)}
