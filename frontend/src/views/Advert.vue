<template>
  <p v-if="error">Something went wrong... {{ error.message }}</p>
  <p v-if="loading">Loading...</p>
  <section class="advert" v-else>
    <div class="up">
      <div class="image-container">
        <img src="https://30.img.avito.st/image/1/1.zjCnc7a4YtmR2qDct2SnJbbRYN8Z0uDR0ddg2xfaatMR.xajaZfbTIv0ViaQVJN-mahjP5knob4APgvyLH6pQDCU" width="300" height="300" alt="">
      </div>
      <div class="info">
        {{ result.getAdvert.title }}
        {{ result.getAdvert.price }} €
      </div>
    </div>
    <div class="down"></div>
    {{ result }}
  </section>
  </template>

<style scoped>
.advert {
  margin: 100px 150px;
  
}


.up {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-direction: row;
}


</style>

<script>
import { defineComponent } from 'vue';
import { useQuery } from '@vue/apollo-composable'
import { GET_ADVERT } from "@/graphql/advert";
import { useRoute } from 'vue-router';


export default defineComponent({
  name: 'App',
  setup() {

    const route = useRoute()

    let id = parseInt(route.params.id);


    const { result, loading, error } = useQuery(GET_ADVERT, {id});

    return {
      result,
      loading,
      error,
    };
  },
});
</script>
