<template>
  <div v-if="loading"><loading></loading></div>
  <div v-else-if="error"><error v-bind="error"/></div>
  <div v-else>
    <div class="title" v-if="!error"> <h1>Recently Added:</h1></div>
    <section class="adverts" v-if="logedIn">
      <adverts @update:isFavorited="updateIsFavorited" v-for="advert in adverts.list" v-bind:key="advert.id" v-bind="advert" />
    </section>
    <section class="adverts" v-else>
      <adverts @update:isFavorited="updateIsFavorited" v-for="advert in adverts.list" v-bind:key="advert.id" v-bind="advert" />
    </section>
  </div>
</template>

<script lang="ts">
import { defineComponent, reactive, watchEffect } from 'vue';
import { useQuery } from '@vue/apollo-composable'
import { GET_ADVERTS, GET_ADVERTS_LOGED } from "@/graphql/advert";
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

    const logedIn = localStorage.getItem('logedIn') === 'true';

    const updateIsFavorited = (id: number, isFavorited: boolean) => {
      const index = adverts.list.findIndex((advert) => advert.id === id);
      const updatedAdverts = [...adverts.list];
      updatedAdverts[index] = { ...updatedAdverts[index], isFavorited };
      adverts.list = updatedAdverts;
    };

    if (logedIn) {
      const accessToken = localStorage.getItem("access_token");
      const { result, loading, error } = useQuery(GET_ADVERTS_LOGED, { accessToken }, { fetchPolicy: 'network-only' });

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
      const { result, loading, error } = useQuery(GET_ADVERTS, { fetchPolicy: 'network-only' });

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

</style>



<!-- <template>
  <div v-if="error" class="error">
    <error  v-bind="error"/>
  </div>
  <p v-if="loading">Loading...</p>
  <div v-else>
    <div class="title" v-if="!error"> <h1>Recently Added:</h1></div>
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
import Error from '../components/Error.vue';

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
  },
  setup() {
    const adverts = reactive({ list: [] as Advert[] });
    const logedIn = localStorage.getItem('logedIn') === 'true';

    const updateIsFavorited = (id: number, isFavorited: boolean) => {
      const index = adverts.list.findIndex((advert) => advert.id === id);
      const updatedAdverts = [...adverts.list];
      updatedAdverts[index] = { ...updatedAdverts[index], isFavorited };
      adverts.list = updatedAdverts;
      console.log
    };

    if (logedIn) {
      const accessToken = localStorage.getItem("access_token");
      const { result, loading, error } = useQuery(GET_ADVERTS_LOGED, { accessToken }, { fetchPolicy: 'network-only' });

      watchEffect(() => {
        if (result.value) {
          adverts.list = [...result.value.getAdvertsLoged];
          adverts.list.sort((a, b) => new Date(b.createdAt).getDate() - new Date(a.createdAt).getDate());
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
      const { result, loading, error } = useQuery(GET_ADVERTS, { fetchPolicy: 'network-only' });

      watchEffect(() => {
      if (result.value) {
        adverts.list = [...result.value.getAdvertsLoged];
        adverts.list.sort((a, b) => new Date(b.createdAt).getDate() - new Date(a.createdAt).getDate());
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

</style> -->
