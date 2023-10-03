<template>
  
  <adverts />
</template>

<script lang="ts">
import { defineComponent } from 'vue';

// Components
import Adverts from '../components/Adverts.vue';

export default defineComponent({
  name: 'App',
  components: {
    Adverts
  },
});
</script>
