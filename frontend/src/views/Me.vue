
<template>
   <p v-if="error">Something went wrong... {{ error.message }}</p>
   <p v-if="loading">Loading...</p>
  <section class="profile" v-else>


    <section class="info">
      <div class="avatar">
        <img src="https://30.img.avito.st/image/1/1.zjCnc7a4YtmR2qDct2SnJbbRYN8Z0uDR0ddg2xfaatMR.xajaZfbTIv0ViaQVJN-mahjP5knob4APgvyLH6pQDCU" alt="">
      </div>
      <div class="description">
        <p class="full-name">
          {{ result.me.name }} {{ result.me.surname }}
        </p>
      </div>

    </section>
    <section class="adverts">
      <div class="switcher">
        <button>New</button>
        <button>Finished</button>
      </div>
      <div class="adverts-container">

      </div>
    </section>
    <div >


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

.avatar {
 
}

.avatar img {
  width: 300px;
  height: 300px;
  border-radius: 50%;
}

</style>

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


