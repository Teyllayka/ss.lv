<template>
  
  <adverts />
</template>

<script lang="ts">
import { defineComponent } from 'vue';

// Components
import HelloWorld from '../components/HelloWorld.vue';
import Adverts from '../components/Adverts.vue';

export default defineComponent({
  name: 'App',
  components: {
    HelloWorld,
    Adverts
  },
});
</script>
