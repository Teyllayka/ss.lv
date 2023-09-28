<template>
<v-container class="fill-height" background="background">
    <p v-if="error">Something went wrong... {{ error.message }}</p>
    <div v-if="loading">

      <p>Loading...</p>
    </div>
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
</v-container>
</template>


<script lang="ts">
import { defineComponent } from 'vue';
import { useQuery } from '@vue/apollo-composable'
import { GET_ADVERTS } from "@/graphql/advert";


export default defineComponent({
  name: 'App',
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