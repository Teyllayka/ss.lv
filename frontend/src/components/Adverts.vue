
<script lang="ts">
import { defineComponent } from 'vue';
import { useQuery } from '@vue/apollo-composable'
import { GET_ADVERTS } from "@/graphql/advert";


export default defineComponent({
  name: 'App',
  props: ['location', 'price', 'id', 'title'],
  setup() {
    const { result, loading, error } = useQuery(GET_ADVERTS);

    console.log(result)

    return {
      result,
      loading,
      error,
    };
  },
});
</script>

<template>

  
<div class="advert-container">
  <router-link :to="'/advert/' + id" class="link"><img src="https://30.img.avito.st/image/1/1.zjCnc7a4YtmR2qDct2SnJbbRYN8Z0uDR0ddg2xfaatMR.xajaZfbTIv0ViaQVJN-mahjP5knob4APgvyLH6pQDCU" width="100" height="100+" alt=""></router-link>
  
  <div class="description">
    <div class="title">{{ title }}</div>
    <div class="price"> {{ price.toFixed(2) }} €</div>
    <div class="details">{{ location }}</div>
    <div class="details">4 september 11:20pm</div>
  </div>
</div>





<!-- <v-container class="fill-height" background="background">
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
</v-container> -->
</template>

<style scoped>


.advert-container {
  width: 370px;
  height: 370px;
  border: 1px solid black;
}

.advert-container .description {
  display: flex;
  justify-content: center;
  align-items: flex-start;
  flex-direction: column;
}
.advert-container .title {
  word-wrap: break-word;
  text-align: left;
  color: rgb(var(--v-theme-text));
  font-size:20px;
}


.advert-container img {
  width: 100%;
  height: 170px;
}


</style>


