
<template>
   <p v-if="error">Something went wrong... {{ error.message }}</p>
   <p v-if="loading">Loading...</p>
   <section class="profile" v-else>


    <section class="info">
      <div class="avatar">
        <img src="https://30.img.avito.st/image/1/1.zjCnc7a4YtmR2qDct2SnJbbRYN8Z0uDR0ddg2xfaatMR.xajaZfbTIv0ViaQVJN-mahjP5knob4APgvyLH6pQDCU" alt="">
      </div>
      <div class="description">
        <div class="data">
          <h1 class="full-name">
            {{ result.me.name }} {{ result.me.surname }}
          </h1>
          <p class="email">
            {{ result.me.email }}
          </p>
          <p>
            {{ result.me.phone }}
          </p>
        </div>
        
        <div class="buttons">
          <button><router-link to="/logout" class="link">Edit</router-link></button>
          <button><router-link to="/logout" class="link">Logout</router-link></button>
        </div>
      </div>

    </section>
    <section class="adverts">
      <div class="switcher">
        <button @click="filterType = 'new'">New</button>
        <button @click="filterType = 'finished'">Finished</button>
      </div>
      <div class="adverts-container">
        <adverts  v-for="advert in filteredAdverts" v-bind:key="advert.id" v-bind="advert" />
      </div>
    </section>
    <div >
    </div>
  </section>
  
</template>


<style scoped>

.profile {
  margin: 100px 150px;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}

.info {
  width: 100%;
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-direction: row;
}

.switcher {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: row;
  gap:20px;
  margin-left:30px;
  margin-bottom:100px;
}

.switcher button {
  padding: 10px 60px;
}
 

.avatar img {
  width: 500px;
  height: 500px;
  border-radius: 50%;
}

.description {
  height:507px;
  width:50%;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-direction: column;
}

.data {
  text-align: start;
}

.full-name {
  margin-top:30px;
  font-size: 32px;
  color: rgb(var(--v-theme-text));
  transition: color 0.1s ease;
}

.description p {
  font-size:20px;
  transition: 0.5s ease;
}

.buttons {
  display: flex;
  justify-content: center;
  align-items: center;
  gap:20px;
  justify-self: flex-end;
  align-self: center;
}

.buttons button {
  padding: 10px 60px;
}

.buttons a {
  color: rgb(var(--v-theme-background));
}

.adverts {
  margin-top: 200px;
 
}

.adverts-container {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-wrap: wrap;
  flex-direction: row;
  gap:20px 20px;
}

</style>

<script lang="ts">
import { defineComponent, computed, ref } from 'vue';
import { ME } from "@/graphql/user";
import { useQuery } from '@vue/apollo-composable'
import Adverts from '../components/Adverts.vue';

export default defineComponent({
 name: 'App',
 components: {
   Adverts
 },
 setup() {
   const accessToken = localStorage.getItem("access_token");
   const { result, loading, error } = useQuery(ME, { accessToken });

   const filterType = ref('new');
   
   // Computed property for new adverts
   const newAdverts = computed(() => {
     if (result.value && result.value.me && result.value.me.adverts) {
       return [...result.value.me.adverts].sort((a, b) => b.id - a.id);
     }
     return [];
   });

   // Computed property for finished adverts
   const finishedAdverts = computed(() => {
     if (result.value && result.value.me && result.value.me.adverts) {
       return result.value.me.adverts.filter((advert: { available: boolean; }) => advert.available === false);
     }
     return [];
   });

   const filteredAdverts = computed(() => {
     return filterType.value === 'new' ? newAdverts.value : finishedAdverts.value;
   });

   return {
     result,
     error,
     loading,
     newAdverts,
     finishedAdverts,
     filterType,
     filteredAdverts
   };
 },
});
</script>