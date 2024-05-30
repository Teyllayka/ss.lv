

<template>
   <section class="stats-container">
      <h1>Statistics</h1>

      <div v-if="loading"><loading></loading></div>
      <div v-else-if="error"><error v-bind="error"/></div>

      <div class="stats" v-else>
         <div class="adverts">
            <p class="desc">Adverts created:</p>
            <p class="value">{{ result.stats.advertCount }}</p>
            <p class="desc">Adverts created today:</p>
            <p class="value">{{ result.stats.todayAdvertCount }}</p>
         </div>
         <div class="users">
            <p class="desc">Users registered:</p>
            <p class="value">{{ result.stats.userCount }}</p>
            <p class="desc">Users registered today:</p>
            <p class="value">{{ result.stats.todayUserCount }}</p>
         </div>
      </div>
   </section>
   



</template>


<style scoped>

.stats-container {
   display: flex;
   justify-content: center;
   align-items: center;
   flex-direction: column;
   gap: 50px;

}

h1 {
   color: rgb(var(--v-theme-text));
   font-size:48px;
}


p.desc {
   font-size: 24px;

}

p.value {
   color: rgb(var(--v-theme-text));
   font-size: 24px;

}

.stats {
   display: flex;
   justify-content: center;
   align-items: center;
   flex-direction: row;
   gap:50px;

   
}

.adverts, .users {
      display: flex;
      justify-self: center;
      align-items: center;
      flex-direction: column;
}

</style>

<script lang="ts">
import { defineComponent } from 'vue';
import { STATS } from "@/graphql/stats";
import { useQuery } from '@vue/apollo-composable'
import Error from '../components/Error.vue';
import Loading from '../components/Loading.vue';

export default defineComponent({
 name: 'App',
 components: {
   Error,
   Loading,
 },
 setup() {
   const { result, loading, error } = useQuery(STATS);

   return {
     result,
     error,
     loading,
   };
 },
});

</script>