<script lang="ts" >
import { defineComponent } from 'vue';

import Header from '@/components/Header.vue';
import Footer from '@/components/Footer.vue';

export default {
  name: 'app',
  components: {
    "myheader": Header,
    "myfooter": Footer,
  }
}
</script>

<template>
  <myheader v-if="!$route.meta.hideNavigation"/>
  <router-view />
  <myfooter v-if="!$route.meta.hideNavigation" />
</template>



