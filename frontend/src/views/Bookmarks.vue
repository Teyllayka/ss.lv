
<script lang="ts">
import { defineComponent } from 'vue';
import { GET_FAVORITES } from "@/graphql/advert";
import { useQuery } from '@vue/apollo-composable'
import Error from '../components/Error.vue';
import Adverts from '../components/Adverts.vue';

export default defineComponent({
  name: 'App',
  components: {
    Adverts,
    Error
  },
  setup() {
        const accessToken = localStorage.getItem("access_token");
        const { result, loading, error } = useQuery(GET_FAVORITES, { accessToken });

        return {
          result,
          error,
          loading
        };
      
    },
});
</script>




<template>  
   <div v-if="error"><error v-bind="error"/></div>
   <p v-if="loading">Loading...</p>
   <div v-else class="adverts-container">
      <div class="title"> <h1>Bookmarks:</h1></div>
      <section class="adverts">
        <adverts  v-for="advert in result.getFavorites" v-bind:key="advert.id" v-bind="advert" />mnbvvvjk bvnghvbcb
      </section>
   </div>
</template>


<style scoped>


.adverts {
    display:flex;
    justify-content:flex-start;
    align-items:center;
    flex-direction: row;
    flex-wrap:wrap;
    gap:30px 30px;
    margin:50px 150px;
  }

.title {
  display: grid;
}

h1 {
    color: rgb(var(--v-theme-text));
    margin: 50px 150px 20px 150px;
    justify-self: flex-start;
  }

</style>