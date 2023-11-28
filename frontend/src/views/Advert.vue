<template>
  <div v-if="loading"><loading></loading></div>
  <div v-else-if="error"><error v-bind="error"/></div>
  <section class="advert" v-else-if="result.getAdvert">
    <h1 class="title"> {{ result.getAdvert.advert.category  }}</h1>
   
    <div class="up">
      <div class="image-container">
        <img src="https://30.img.avito.st/image/1/1.zjCnc7a4YtmR2qDct2SnJbbRYN8Z0uDR0ddg2xfaatMR.xajaZfbTIv0ViaQVJN-mahjP5knob4APgvyLH6pQDCU" width="300" height="300" alt="">
      </div>
      <div class="info">
        <h1 class="title"> {{ result.getAdvert.advert.title }}</h1>
        <p class="price"> {{ result.getAdvert.advert.price }} €</p>
       
        <p class="posted">posted {{ getDate(result.getAdvert.advert.createdAt) }} by <router-link :to="'/user/' + result.getAdvert.user.id" class="link">{{ result.getAdvert.user.name }} {{ result.getAdvert.user.surname }}</router-link></p>
        <p>you can contact him by phone {{ result.getAdvert.user.phone }} or by email {{ result.getAdvert.user.email }}</p>

        <div class="buttons">
          <button>Message</button>
        </div>
      </div>
    </div>
    <h1 class="title">Characteristics:</h1>
    <div class="down">


    </div>
    <h1 class="title">Description:</h1>
    <div class="down">
      {{ result.getAdvert.advert.description }}
    </div>
  </section>
  <section class="advert" v-else-if="result.getAdvertLoged">
    <h1 class="title"> {{ result.getAdvertLoged.advert.category  }}</h1>
   
    <div class="up">
      <div class="image-container">
        <img src="https://30.img.avito.st/image/1/1.zjCnc7a4YtmR2qDct2SnJbbRYN8Z0uDR0ddg2xfaatMR.xajaZfbTIv0ViaQVJN-mahjP5knob4APgvyLH6pQDCU" width="300" height="300" alt="">
      </div>
      <div class="info">
        <h1 class="title"> {{ result.getAdvertLoged.advert.title }}</h1>
        <p class="price"> {{ result.getAdvertLoged.advert.price }} €</p>
       
        <p class="posted">posted {{ getDate(result.getAdvertLoged.advert.createdAt) }} by <router-link :to="'/user/' + result.getAdvertLoged.user.id" class="link">{{ result.getAdvertLoged.user.name }} {{ result.getAdvertLoged.user.surname }}</router-link></p>
        <p>you can contact him by phone {{ result.getAdvertLoged.user.phone }} or by email {{ result.getAdvertLoged.user.email }}</p>

        <div class="buttons">
          <button>Message</button>
          <button v-if="isFavorited && logedIn" @click="removeFromFavorite">Remove from favorites</button>
          <button v-else-if="!isFavorited && logedIn" @click="addToFavorite">Add to favorites</button>
        </div>
      </div>
    </div>
    <h1 class="title">Characteristics:</h1>
    <div class="down">


    </div>
    <h1 class="title">Description:</h1>
    <div class="down">
      {{ result.getAdvertLoged.advert.description }}
      {{ result }}
    </div>
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

img {
  width:500px;
  height: 500px;
}

.info {
 width: 50%;
 display: flex;
 justify-content: flex-start;
 align-items: flex-start;
 flex-direction: column;
 height: 507px;
}

.info .buttons {
  margin-top:100px;
  width:100%;
  display: flex;
  justify-content: center;
  align-items: center;
}

.buttons {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: row;
  gap:30px;
}

.buttons button {
   
   padding: 10px 30px;
   border-radius: 50px;
   color: rgb(var(--v-theme-background));
}

.title {
  color: rgb(var(--v-theme-text));
  font-size:32px;
}

.price {
  font-size: 20px;
}

.posted {
  font-size: 20px;
}

.down {
  margin-top:100px;
}


</style>

<script>
import { defineComponent, ref, watch } from 'vue';
import { useQuery } from '@vue/apollo-composable'
import { GET_ADVERT, GET_ADVERT_LOGED } from "@/graphql/advert";
import { useRoute } from 'vue-router';
import Error from '../components/Error.vue';
import Loading from '../components/Loading.vue';
import { ADD_FAVORITE, REMOVE_FAVORITE } from "@/graphql/advert";
import { useMutation } from '@vue/apollo-composable';
import { useApolloClient } from '@vue/apollo-composable'




export default defineComponent({
  name: 'App',
  components: {
    Error,
    Loading
  },
  setup() {
    const route = useRoute()

    const isFavorited = ref(false);

    let id = parseInt(route.params.id);

    const logedIn = localStorage.getItem('logedIn') === 'true';

    const getDate = function(timestamp) {
      let date = new Date(timestamp);
      let monthNames = ["January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
      ];

      let day = date.getDate();
      let month = monthNames[date.getMonth()];
      let hours = date.getHours();
      let minutes = date.getMinutes();
      let period = hours >= 12 ? 'pm' : 'am';
      hours = hours % 12;
      hours = hours ? hours : 12; 
      minutes = minutes < 10 ? '0'+minutes : minutes;
      let strTime = hours + ':' + minutes + period;

      let today = new Date();
      let yesterday = new Date(Date.now() - 24 * 60 * 60 * 1000);

      let formattedDate = '';
      if (date.toDateString() === today.toDateString()) {
        formattedDate = 'Today ' + strTime;
      } else if (date.toDateString() === yesterday.toDateString()) {
        formattedDate = 'Yesterday ' + strTime;
      } else {
        formattedDate = day + ' ' + month + ' ' + strTime;
      }

      return formattedDate;
    }

    


    if (logedIn) {
      const accessToken = localStorage.getItem("access_token");


      const { result, loading, error, onResult } = useQuery(GET_ADVERT_LOGED, { id, accessToken }, { fetchPolicy: 'network-only' });


      onResult((result) => {
        isFavorited.value = result.data.getAdvertLoged.advert.isFavorited;
        console.log(isFavorited);
      });


      const { mutate: addFavorite } = useMutation(ADD_FAVORITE);
      const { mutate: removeFavorite } = useMutation(REMOVE_FAVORITE);

      const addToFavorite = async function() {
          await addFavorite({
            accessToken,
            advertId: id
            
          }).then(({ data, loading, error }) => {
            if (error) {
              console.error(`An error occurred: ${error.message}`);
              return;
            }

            console.log(data);
          
          });

          isFavorited.value = true;
      }

      const removeFromFavorite = async function() {
          await removeFavorite({
            accessToken,
            advertId: id
            
          }).then(({ data, loading, error }) => {
            if (error) {
              console.error(`An error occurred: ${error.message}`);
              return;
            }

            console.log(data);
          
          });
          if (this.$route.name == "Bookmarks") {
            console.log("bookmarks");
            document.getElementById(id).remove();
            
          }

          isFavorited.value = false;
      }

      return {
        result,
        loading,
        error,
        getDate,
        addToFavorite,
        removeFromFavorite,
        isFavorited,
        logedIn
      };

    } else {
      const { result, loading, error } = useQuery(GET_ADVERT, {id}, { fetchPolicy: 'network-only' });

      return {
        result,
        loading,
        error,
        getDate,
        logedIn
      };

    }

  },
});
</script>
