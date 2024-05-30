<template>

  <div v-if="loading"><loading></loading></div>
  <div v-else-if="error"><error v-bind="error"/></div>
  <div v-else>
    <div class="title" v-if="!error"> <h1>Recently Added:</h1></div>
    <section class="adverts" >
      
      <adverts @update:isFavorited="updateIsFavorited" v-for="advert in adverts.list" v-bind:key="advert.id" v-bind="advert" />
      <div class="scroll-trigger" ref="scrollTrigger"></div>
    </section>
    <div v-if="!hasMore" class="end">That's all for now</div>

  </div>
</template>
<script lang="ts">
import {defineComponent, reactive, watchEffect, onMounted, onUnmounted, ref } from 'vue';
import { useQuery } from '@vue/apollo-composable'
import { GET_ADVERTS } from "@/graphql/advert";
import Adverts from '../components/Adverts.vue';
import Error from '../components/Error.vue';
import Loading from '../components/Error.vue';

interface Advert {
 id: number;
 location: string;
 price: number;
 title: string;
 createdAt: string;
 available: boolean;
 isFavorited: boolean;
}

export default defineComponent({
 name: 'App',
 components: {
    Adverts,
    Error,
    Loading,
 },
  
 setup() {
    const adverts = reactive({ list: [] as Advert[] });
    let offset = 0;
    const limit = 8;
    const accessToken = localStorage.getItem("access_token") || "";
    const hasMore = ref(true);

    const updateIsFavorited = (id: number, isFavorited: boolean) => {
      const index = adverts.list.findIndex((advert) => advert.id === id);
      const updatedAdverts = [...adverts.list];
      updatedAdverts[index] = { ...updatedAdverts[index], isFavorited };
      adverts.list = updatedAdverts;
    };

    const {result, loading, error, fetchMore} = useQuery(GET_ADVERTS, { accessToken, offset, limit }, { fetchPolicy: 'network-only' });

    watchEffect(() => {
      if (result.value) {
        adverts.list = result.value.getAdverts;
      } 
    });

    const handleScroll = () => {
      let bottomOfWindow = document.documentElement.scrollTop + window.innerHeight >= document.documentElement.offsetHeight - 200;
      console.log( document.documentElement.scrollTop + window.innerHeight, document.documentElement.offsetHeight - 200)
      if (bottomOfWindow && hasMore.value) {
          offset += limit;

          fetchMore({
            variables: {
              offset: offset,
              limit: limit,
              accessToken: accessToken,
            },
       
          }).then(a => {
            adverts.list = [...adverts.list, ...a.data.getAdverts];
            hasMore.value = a.data.getAdverts.length == limit;
            console.log("Fetched more adverts:", a, hasMore.value);
          
          }).catch(error => {
            console.error("Error fetching more adverts:", error);
          });
      }
    };


    onMounted(() => {
      window.addEventListener("scroll", handleScroll);
    });

    onUnmounted(() => {
      window.removeEventListener("scroll", handleScroll);
    });

    

    return {
      adverts,
      loading,
      error,
      updateIsFavorited,
      hasMore
    };
 },
});
</script>




<style scoped>
  .adverts {
    display:flex;
    justify-content:flex-start;
    align-items:center;
    flex-direction: row;
    flex-wrap:wrap;
    gap:30px 30px;
    margin:50px 150px;
  }

  .error {
    height:100%;
  }

  .title {
    display:grid;
  }

  h1 {
    color: rgb(var(--v-theme-text));
    margin: 50px 150px 20px 150px;
    justify-self: flex-start;
  }

  .end {
    display: flex;
    justify-content: center;
    align-items: center;
    width:100%;
    margin: 50px 0px;
  }

</style>


