<template>
   <div class="container">
   <div>

   <div class="filters">
      <div>Select: <select v-model="selectedCategory">
      <option v-for="category in categories" :key="category" :value="category">
         {{ category }}
      </option>
      <option value=""></option>
   </select>
   <div>Name: <input type="text" v-model="selectedName"></div>
</div>
   
   <div>Sort by: 
      <select v-model="sortOption">
         <option value="">Select Sorting Option</option>
         <option value="price_asc">Price - Ascending</option>
         <option value="price_desc">Price - Descending</option>
         <option value="date_created_asc">Date Created - Ascending</option>
         <option value="date_created_desc">Date Created - Descending</option>
         <option value="name">Name</option>
      </select>
   </div>
   <div v-if="selectedCategory && categoryFields[selectedCategory]">
   <div class="cond" v-for="(field, index) in categoryFields[selectedCategory]" :key="index">
      <label :for="'filter-' + field">{{ field }}</label>
      <input :id="'filter-' + field" v-model="filterValues[field]" placeholder="Filter by..." />
   </div>
   </div>



   </div>
   
  
   <div v-if="loading"><loading></loading></div>
   <div v-else-if="error"><error v-bind="error"/></div>
   <div v-else>
      <section class="adverts">
         <adverts  v-for="advert in adverts" v-bind:key="advert.id" v-bind="advert" />
      </section>
   </div>
   </div>
   </div>
</template>


<style scoped>

@media only screen and (max-width: 1400px) {
  
   .container {
      margin:50px 20px !important;
   }
}

.cond {
   display: flex;
   justify-content: flex-start;
   align-items: center;
   gap:20px;
   
}

option, input {
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
      margin-top: 30px;
   }
</style>
<script>
import { ref, watch, reactive } from 'vue';
import { useQuery } from '@vue/apollo-composable';
import { GET_ADVERTS } from "@/graphql/advert";
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
    const selectedName = ref('');
    const sortOption = ref('');
    const displayedAdverts = ref([]);
    const filterValues = reactive({});



    const categories = ['Cars', 'Electronics', 'Furniture', 'Clothes', 'Books', 'Other'];
    const accessToken = localStorage.getItem("access_token") || "";
    const categoryFields = {
      Cars: ['model', 'year', 'fuelType'],
      Electronics: ['brand', 'model', 'condition'],
      Furniture: ['material', 'size', 'color'],
      Clothes: ['size', 'color', 'type'],
      Books: ['author', 'genre', 'publicationYear'],
      Other: []
   };

    const {result, loading, error} = useQuery(GET_ADVERTS, { accessToken, offset: 0, limit: 999999 }, { fetchPolicy: 'network-only' });

    watch([result, sortOption, selectedCategory, selectedName, filterValues], ([newResult]) => {


      let filteredAdverts = [...newResult.getAdverts];
      let sortedAdverts = [];

      
      if (selectedName.value!== '') {
         console.log(filteredAdverts)
         filteredAdverts = filteredAdverts.filter(advert => advert.title.toLowerCase().includes(selectedName.value.toLowerCase()));
      }

      if (selectedCategory.value!== '') {
         console.log(filteredAdverts)
        filteredAdverts = filteredAdverts.filter(advert => advert.category === selectedCategory.value);
      }


      for (const [key, value] of Object.entries(filterValues)) {
        if (value !== '') {
            console.log(key, value, filterValues);
          //filteredAdverts = filteredAdverts.filter(advert => advert[key].toLowerCase().includes(value.toLowerCase()));
        }
      }



      if (sortOption.value!== '') {
        sortedAdverts = filteredAdverts.sort((a, b) => {
          if (sortOption.value.includes('price')) {
            if (sortOption.value.startsWith('price_asc')) return a.price - b.price;
            if (sortOption.value.startsWith('price_desc')) return b.price - a.price;
          } else if (sortOption.value.includes('date_created')) {
            if (sortOption.value.startsWith('date_created_asc')) return new Date(a.createdAt) - new Date(b.createdAt);
            if (sortOption.value.startsWith('date_created_desc')) return new Date(b.createdAt) - new Date(a.createdAt);
          } else if (sortOption.value === 'name') {
            return String(a.name).localeCompare(String(b.name));
          }
          return 0;
        });
      } else {
        sortedAdverts = filteredAdverts;
      }

      

      displayedAdverts.value = sortedAdverts;
    });

    return { adverts: displayedAdverts, selectedCategory, categories, loading, error, sortOption, categoryFields, filterValues, selectedName };
  },
};
</script>