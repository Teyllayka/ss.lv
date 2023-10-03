<template>
  

<section class="adverts">
  <div class="advert-container" v-for="i in [1,2,3,4,5,6,7,8,9,10,11,12,13]" v-bind:key="i">
    <img src="https://30.img.avito.st/image/1/1.zjCnc7a4YtmR2qDct2SnJbbRYN8Z0uDR0ddg2xfaatMR.xajaZfbTIv0ViaQVJN-mahjP5knob4APgvyLH6pQDCU" width="100" height="100+" alt="">
    <div class="description">
      <div class="title">kia rio car 2023 year lol lol lol</div>
      <div class="price">100.23 €</div>
      <div class="details">Riga, Latvia</div>
      <div class="details">4 september 11:20pm</div>
    </div>
  </div>




</section>






<v-container class="fill-height" background="background">
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
</v-container>
</template>

<style scoped>

.adverts {
  margin:50px 100px;
  display: flex;
  justify-content: flex-start;
  align-items: center;
  flex-direction: row;
  flex-wrap: wrap;
  gap: 40px 40px;
}


.advert-container {
  width: 200px;
  height: 200px;
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
}


.advert-container img {
  width: 100%;
}


</style>


<script lang="ts">
import { defineComponent } from 'vue';
import { useQuery } from '@vue/apollo-composable'
import { GET_ADVERTS } from "@/graphql/advert";


export default defineComponent({
  name: 'App',
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