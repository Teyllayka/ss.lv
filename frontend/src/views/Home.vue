<template>
  <div class="title"> <h1>Recently Added:</h1></div>
 
  <section class="adverts">
    <adverts />
  </section>
</template>


<style scoped>
  .adverts {
    display:flex;
    justify-content:flex-start;
    align-items:center;
    flex-direction: row;
    flex-wrap:wrap;
    gap:40px 40px;
    margin:50px 150px;
  }

  .title {
    display:grid;
  }

  h1 {
   color: rgb(var(--v-theme-text));
    margin: 50px 150px 20px 150px;
    justify-self: flex-start;
  }

</style>


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
