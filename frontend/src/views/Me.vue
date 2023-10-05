
<template>

  <p v-if="error">Something went wrong... {{ error.message }}</p>
  <p v-if="loading">Loading...</p>
  <div v-else>

    {{ result }}
   
    <!-- <p> email: {{ result.me.email }}</p>
    <p> name: {{ result.me.name }}</p>
    <p>surname: {{ result.me.surname }}</p>
    

    <p> adverts:</p>
   
    <ul>
      <li v-for="advert in result.me.adverts" v-bind:key="advert.id">
        <div>
          location: {{ advert.location }}
          price: {{ advert.price }}
        </div>
      </li>
    </ul -->
  </div>
  
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { ME } from "@/graphql/user";
import { useQuery } from '@vue/apollo-composable'

export default defineComponent({
  name: 'App',
  setup() {
        const accessToken = localStorage.getItem("access_token");
        const { result, loading, error } = useQuery(ME, { accessToken });

        return {
          result,
          error,
          loading
        };
      
    },
});
</script>


