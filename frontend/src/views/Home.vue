<template>
  <p v-if="error">Something went wrong... {{ error.message }}</p>
  <p v-if="loading">Loading...</p>
  <div v-else>
    <div class="title"> <h1>Recently Added:</h1></div>
    <section class="adverts" v-if="logedIn">
      <adverts @update:isFavorited="updateIsFavorited" v-for="advert in adverts.list" v-bind:key="advert.id" v-bind="advert" />
    </section>
    <section class="adverts" v-else>
      <adverts @update:isFavorited="updateIsFavorited" v-for="advert in adverts.list" v-bind:key="advert.id" v-bind="advert" />
    </section>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, reactive, watchEffect } from 'vue';
import { useQuery } from '@vue/apollo-composable'
import { GET_ADVERTS, GET_ADVERTS_LOGED } from "@/graphql/advert";
import Adverts from '../components/Adverts.vue';

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
    Adverts
  },
  setup() {
    const adverts = reactive({ list: [] as Advert[] });
    const logedIn = localStorage.getItem('logedIn') === 'true';

    const updateIsFavorited = (id: number, isFavorited: boolean) => {
      const index = adverts.list.findIndex((advert) => advert.id === id);
      const updatedAdverts = [...adverts.list];
      updatedAdverts[index] = { ...updatedAdverts[index], isFavorited };
      adverts.list = updatedAdverts;
    };



    if (logedIn) {
      const accessToken = localStorage.getItem("access_token");
      const { result, loading, error } = useQuery(GET_ADVERTS_LOGED, { accessToken });

      watchEffect(() => {
        if (result.value) {
          adverts.list = result.value.getAdvertsLoged;
        }
      });

      return {
        adverts,
        loading,
        error,
        logedIn,
        updateIsFavorited
      };
    } else {
      const { result, loading, error } = useQuery(GET_ADVERTS);

      watchEffect(() => {
        if (result.value) {
          adverts.list = result.value.getAdverts;
        }
      });

      return {
        adverts,
        loading,
        error,
        logedIn,
        updateIsFavorited
      };
    }
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

  .title {
    display:grid;
  }

  h1 {
    color: rgb(var(--v-theme-text));
    margin: 50px 150px 20px 150px;
    justify-self: flex-start;
  }

</style>
