
<script lang="ts">
import { defineComponent } from 'vue';
import { GET_FAVORITES } from "@/graphql/advert";
import { useQuery } from '@vue/apollo-composable'
import Error from '../components/Error.vue';
import Adverts from '../components/Adverts.vue';
import Loading from '../components/Loading.vue';

export default defineComponent({
  name: 'App',
  components: {
    Adverts,
    Error,
    Loading
  },
  setup() {
        const accessToken = localStorage.getItem("access_token");
        const { result, loading, error } = useQuery(GET_FAVORITES, { accessToken }, { fetchPolicy: 'network-only' });

        
      

        return {
          result,
          error,
          loading
        };
      
    },
});
</script>




<template>  
   <div v-if="loading"><loading></loading></div>
  <div v-else-if="error"><error v-bind="error"/></div>
   <div v-else class="adverts-container">
      <div class="title"> <h1>Bookmarks:</h1></div>
      <section class="adverts">
        <adverts  v-for="advert in result.getFavorites" v-bind:key="advert.id" v-bind="advert" />
      </section>
   </div>
</template>


<style scoped>

@media (max-width: 1400px) {
  h1 {
      text-align: center !important;
      margin: 50px 20px !important;
  }

}


.adverts {
    display:flex;
    justify-content:flex-start;
    align-items:center;
    flex-direction: row;
    flex-wrap:wrap;
    gap:30px 30px;
    margin:50px 20px;
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