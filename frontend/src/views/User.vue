<template>
  <p v-if="error">Something went wrong... {{ error.message }}</p>
  <p v-if="loading">Loading...</p>
  <div v-else>
   
    <p> email: {{ result.getUser.email }}</p>
    <p> name: {{ result.getUser.name }}</p>
    <p>surname: {{ result.getUser.surname }}</p>
    

    <p> adverts:</p>
   
    <ul>
      <li v-for="advert in result.getUser.adverts" v-bind:key="advert.id">
        <div>
          location: {{ advert.location }}
          price: {{ advert.price }}
        </div>
      </li>
    </ul>
  </div>
  <div></div>
</template>

<script>
import { useQuery } from '@vue/apollo-composable'
import { GET_USER } from "../graphql/user";
import { useRoute } from 'vue-router';

export default {
  name: 'App',
  setup () {

    const route = useRoute()

    let id = parseInt(route.params.id);

    const { result, loading, error } = useQuery(GET_USER, { id } );
    return {
      result,
      loading, 
      error,
    }
  }
}

</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
</style>
