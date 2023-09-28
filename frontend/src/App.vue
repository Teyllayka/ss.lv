<template>
  <myheader/>
  <router-view />
</template>

<script lang="ts" >
import { defineComponent } from 'vue';

import Header from '@/components/Header.vue';

export default {
  name: 'app',
  components: {
    "myheader": Header
  }
}
</script>
