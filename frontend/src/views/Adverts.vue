<template>
   <div class="container">
   <div>

   <div class="filters">
      <div>Select: <select v-model="selectedCategory">
    <option v-for="category in categories" :key="category" :value="category">
      {{ category }}
    </option>
   </select></div>
      <div>Sort by: <select v-model="selectedCategory">
         <option value="">1</option>
         <option value="">2</option>
         <option value="">3</option>
   </select></div>
   </div>
   
  
   <div v-if="loading"><loading></loading></div>
   <div v-else-if="error"><error v-bind="error"/></div>
   <div v-else>
      <section class="adverts">
         <adverts  v-for="advert in result.getAdvertsByCategory" v-bind:key="advert.id" v-bind="advert" />
      </section>
   </div>
   </div>
   </div>
</template>


<style scoped>

option {
   background-color: rgb(var(--v-theme-background));
   color: rgb(var(--v-theme-text));
}

select {
   margin-bottom:30px;
   background-color: rgb(var(--v-theme-background));
   color: rgb(var(--v-theme-text));
}

.container {
   margin:50px 150px;
}

   .adverts {
      display: flex;
      justify-content: flex-start;
      align-items: center;
      flex-direction: row;
      flex-wrap: wrap;
      gap:30px 30px;
   }
</style>
  
<script>
   import { ref, watch } from 'vue';
   import { useQuery } from '@vue/apollo-composable';
   import { GET_ADVERTS_CATEGORY } from "@/graphql/advert";
   import Error from '../components/Error.vue';
   import Adverts from '../components/Adverts.vue';
   import Loading from '../components/Loading.vue';

   export default {
   components: {
    Adverts,
    Error,
    Loading
   },
   setup() {
      const selectedCategory = ref('');
      const categories = ['Cars', 'Electronics', 'Furniture', 'Clothes', 'Books', 'Other'];

      const { result, loading, error, refetch } = useQuery(GET_ADVERTS_CATEGORY, { category: selectedCategory.value }, { fetchPolicy: 'network-only' });

      watch(selectedCategory, () => {
         refetch({ category: selectedCategory.value });
      });

      return { selectedCategory, categories, result, loading, error };
   },
   };
</script>
  
  