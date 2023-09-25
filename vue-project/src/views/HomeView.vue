<script setup lang="ts">
import TheWelcome from '../components/TheWelcome.vue'
import gql from 'graphql-tag'
import { useQuery } from '@vue/apollo-composable'
import { GET_ADVERTS } from "../graphql/advert";
const { result, loading, error } = useQuery(GET_ADVERTS);


</script>

<template>
  <main>
    <!-- <TheWelcome /> -->
    <p v-if="error">Something went wrong... {{ error.message }}</p>
    <p v-if="loading">Loading...</p>
    <div v-else>

      <p>Recent adverts:  </p>
      <ul>
        <li v-for="advert in result.getAdverts" v-bind:key="advert.id">
          <div>
            location: {{ advert.location }}
            price: {{ advert.price }}
          </div>
        </li>
      </ul>
      
    </div>
  </main>
</template>

