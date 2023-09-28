<template>
  <hello-world />
  <p v-if="error">Something went wrong... {{ error.message }}</p>
  <p v-if="loading">Loading...</p>
  <div v-else style="color:white;">
    
    <p> adverts:</p>
   
    <ul>
      <li v-for="advert in result.getAdverts" v-bind:key="advert.id">
        <div>
          location: {{ advert.location }}
          price: {{ advert.price }}
        </div>
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useQuery } from '@vue/apollo-composable'
import { GET_ADVERTS } from "@/graphql/advert";

// Components
import HelloWorld from '../components/HelloWorld.vue';

export default defineComponent({
  name: 'App',
  components: {
    HelloWorld,
  },
  setup() {
    const { result, loading, error } = useQuery(GET_ADVERTS);

    return {
      result,
      loading,
      error,
    };
  },
});
</script>
