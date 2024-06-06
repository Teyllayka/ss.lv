
<template>
   <div v-if="loading"><loading></loading></div>
   <div v-else-if="error"><error v-bind="error"/></div>
   <section class="profile" v-else>


    <section class="info">
      <div class="avatar">
        <img :src="result.me.avatarUrl" alt="">
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
          <router-link to="/edit" class="link"><button>Edit</button></router-link>
          <router-link to="/logout" class="link"><button>Logout</button></router-link>
        </div>
      </div>

    </section>
    <section class="adverts" v-if="newAdverts.length > 0">
      <div class="switcher">
        <button @click="filterType = 'new'">New</button>
        <button @click="filterType = 'finished'">Finished</button>
      </div>
      <div class="adverts-container">
        <adverts  v-for="advert in filteredAdverts" v-bind:key="advert.id" v-bind="advert" />
      </div>
    </section>
    <section class="adverts-else" v-else>
      <p>you havent created adverts yet</p>
    </section>
    <div >
    </div>
  </section>
  
</template>


<style scoped>
@media only screen and (max-width: 1400px) {
  .avatar img {
    width:400px !important;
    height:400px !important;
  }

  .description {
    height: 400px !important;
  }
}

@media only screen and (max-width: 1200px) {
  .avatar img {
    width:300px !important;
    height:300px !important;
  }

  .description {
    height: 300px !important;
  }
}

@media only screen and (max-width: 1000px) {
  .info {
    flex-direction: column !important;
    justify-content: center !important;
    align-items: center !important;
  }

  .profile {
    margin: 100px 0px !important;
  }

  .adverts {
    width: 100% !important;
  }

  

  .description {
    width: 100% !important;
    justify-content: center !important;
    align-items: center !important;
  }

  .adverts-container {
    margin-top:100px;
  }

  .buttons, .switcher {
    justify-content: space-around !important;
    align-items: center !important;
    gap:50px !important;
    margin: 0px !important;
  }
}

@media only screen and (max-width: 400px) {
  
  .buttons button {
    padding: 10px 30px !important;
  }
}



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
  align-items: flex-start;
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
  color: rgb(var(--v-theme-background));
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
  width:100%;
  display: flex;
  justify-content: flex-start;
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

.adverts-else {
  margin-top:100px;
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
import Error from '../components/Error.vue';
import Adverts from '../components/Adverts.vue';
import Loading from '../components/Loading.vue';

export default defineComponent({
 name: 'App',
 components: {
   Adverts,
   Error,
   Loading,
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