<template>
  <p v-if="error">Something went wrong... {{ error.message }}</p>
  <p v-if="loading">Loading...</p>
  <section class="profile" v-else>
   <section class="info">
     <div class="avatar">
       <img :src="result.getUser.avatarUrl" alt="">
     </div>
     <div class="description">
       <div class="data">
         <h1 class="full-name">
           {{ result.getUser.name }} {{ result.getUser.surname }}
         </h1>
         <p class="email">
           {{ result.getUser.email }}
         </p>
         <p>
           {{ result.getUser.phone }}
         </p>
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
      <p>This user didnt create any adverts... how did you find it lol?</p>
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
    padding:0px 20px;
  }

  .buttons, .switcher {
    justify-content: space-around !important;
    align-items: center !important;
    margin: 0px !important;
    gap: 0px !important;
  }
}

@media only screen and (max-width: 400px) {
  
  button {
    padding: 10px 30px !important;
  }

  .data {
    text-align: center !important;
  
  }
}

button {
  color: rgb(var(--v-theme-background));

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


.adverts-else {
  margin-top:100px;
}

</style>

<script lang="ts">
import { useQuery } from '@vue/apollo-composable'
import { GET_USER } from "../graphql/user";
import { useRoute } from 'vue-router';
import { computed, ref } from 'vue';


import Adverts from '../components/Adverts.vue';

export default {
  name: 'App',
  components: {
    Adverts
  },
  setup () {

    const route = useRoute()

    let id = Array.isArray(route.params.id) ? route.params.id[0] : parseInt(route.params.id);


    const filterType = ref('new');

    

    const { result, loading, error } = useQuery(GET_USER, { id } );

    const newAdverts = computed(() => {
     if (result.value && result.value.getUser && result.value.getUser.adverts) {
       return [...result.value.getUser.adverts].sort((a, b) => b.id - a.id);
     }
     return [];
    });

    const finishedAdverts = computed(() => {
      if (result.value && result.value.getUser && result.value.getUser.adverts) {
        return result.value.getUser.adverts.filter((advert: { available: boolean; }) => advert.available === false);
      }
      return [];
    });

    const filteredAdverts = computed(() => {
      return filterType.value === 'new' ? newAdverts.value : finishedAdverts.value;
    });


    return {
      result,
      loading, 
      error,
      newAdverts,
      finishedAdverts,
      filterType,
      filteredAdverts
    }
  }
}

</script>

