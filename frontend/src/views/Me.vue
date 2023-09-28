
<template>
  
   <profile />
   <register />

</template>

<script lang="ts">
import { defineComponent } from 'vue';

// Components
import Profile from '../components/Profile.vue';
import Register from '../components/Register.vue';

export default defineComponent({
  name: 'App',
  components: {
    Profile,
    Register
  },
});
</script>
