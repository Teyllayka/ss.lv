<template>
  <div v-if="loading"><loading></loading></div>
  <div v-else-if="error"><error v-bind="error"/></div>
 
  <section class="advert" v-else-if="result.getAdvert">
    <h1 class="title"> {{ result.getAdvert.advert.category  }}</h1>

   
    <div class="up">
      <div class="image-container">
        <img :src="result.getAdvert.advert.photoUrl"  alt="">
      </div>
      <div class="info">
        <div class="data">
          <h1 class="title"> {{ result.getAdvert.advert.title }}</h1>
        <p class="price"> {{ result.getAdvert.advert.price.toFixed(2) }} €</p>
        <p class="location"><span>Location:</span> <span class="highlight">{{ result.getAdvert.advert.location }}</span></p>
       
        <p class="posted">posted {{ getDate(result.getAdvert.advert.createdAt) }} by <router-link :to="'/user/' + result.getAdvert.user.id" class="link">{{ result.getAdvert.user.name }} {{ result.getAdvert.user.surname }}</router-link></p>
        <p>you can contact him by phone <span class="highlight">{{ result.getAdvert.user.phone }}</span> or by email <span class="highlight">{{ result.getAdvert.user.email }}</span></p>
        </div>
        

        <div class="buttons">
          <!-- <button>Message</button> -->
          <button v-if="isFavorited && logedIn" @click="removeFromFavorite">Remove from favorites</button>
          <button v-else-if="!isFavorited && logedIn" @click="addToFavorite">Add to favorites</button>
          <button v-if="logedIn && result.getAdvert.isAdmin == true" @click="deleteAdvertDB">Delete</button>
        </div>
      </div>
    </div>
    <h1 class="title">Characteristics:</h1>
    <div >
      <p v-for="(value, key) in result.getAdvert.advert.specs" :key="key" class="value-key">{{ value.key }}: <span class="value">{{ value.value }}</span></p>
    </div>
    <div class="down">


    </div>
    <h1 class="title">Description:</h1>
    <div class="down">
      {{ result.getAdvert.advert.description }}
    </div>
    <div class="other-photos">
      <img :src="photo" v-for="photo in result.getAdvert.advert.additionalPhotos" v-bind="additionalPhotos" :key="photo" alt="">

    </div>
  </section>
  </template>

<style scoped>
.other-photos {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-direction: row;
}

.value-key {
  display: flex;
  justify-content: flex-start;
  align-items: center;
  flex-direction: row;
  gap:20px;
}

.advert {
  margin: 100px 150px;
  display: flex;
  justify-content: center;
  align-content: center;
  flex-direction: column;
}

.location {
  display: flex;
  justify-content: flex-start;
  align-items: center;
  flex-direction: row;
  max-width: 100%;
  gap:10px;
  text-wrap: wrap;
  white-space: wrap;
  word-wrap: break-word;
  text-overflow: ellipsis; 
  overflow: hidden; 
  word-break: break-all;
}
.highlight {
  max-width: 20ch;
  text-overflow: ellipsis;
  overflow: hidden; 
}

.highlight {
  color: rgb(var(--v-theme-text));
}


.title {
  max-width: 100%;
  text-wrap: wrap;
  white-space: wrap;
  word-break: break-all;
  max-height: 200px;
  text-overflow: ellipsis; 
  overflow: hidden; 
}


.up {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-direction: row;
  margin-bottom:50px;
  height:100%;
  height: 500px;
}

img {
  width:500px;
  height: 500px;
}

.image-container {
  height:100%;
}

@media only screen and (max-width: 1400px) {

  img {
    width:400px !important;
    height: 400px!important;
  }

  .up {
    height: 400px;
  }

}

@media only screen and (max-width: 1200px) {

img {
  width:300px !important;
  height: 300px!important;
}

.up {
    height: 300px;
  }

}

@media only screen and (max-width: 1000px) {

.up {
    height: 100%;
    flex-direction: column;
    width: 100%;
  }
.data, .info, .image-container, img {
  width:100% !important;
}

.info {
  height: calc(100% + 100px) !important;
}

.advert {
  width:90% !important;
  margin: 20px;
}


}

.info {
 width: 50%;
 display: flex;
 justify-content: space-between;
 align-items: flex-start;
 flex-direction: column;
 height: 100%;
}

.data {
  display: flex;
  justify-content: flex-start;
  align-items: flex-start;
  flex-direction: column;
}

.info .buttons {
  width:100%;
  display: flex;
  justify-content: flex-start;
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
  /* color: rgb(var(--v-theme-text)); */
  font-size:32px;
}

.price {
  font-size: 20px;
  color: rgb(var(--v-theme-text));
}

.posted {
  font-size: 20px;
}

.down {
  margin-top:100px;
  max-width: 100%;
  text-wrap: wrap;
  word-wrap: break-word;

}


.value {
  font-weight: bold;
  color: rgb(var(--v-theme-text));
  max-width: 10%;
  text-overflow: ellipsis; 
  overflow: hidden; 
  display: block;
}


</style>

<script>
import { defineComponent, ref } from 'vue';
import { useQuery } from '@vue/apollo-composable'
import { GET_ADVERT } from "@/graphql/advert";
import { useRoute } from 'vue-router';
import Error from '../components/Error.vue';
import Loading from '../components/Loading.vue';
import { ADD_FAVORITE, REMOVE_FAVORITE, DELETE_ADVERT } from "@/graphql/advert";
import { useMutation } from '@vue/apollo-composable';
import { useRouter } from 'vue-router';



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

    const accessToken = localStorage.getItem("access_token") || "";
    const logedIn = localStorage.getItem("logedIn") || "";



    const { result, loading, error, onResult } = useQuery(GET_ADVERT, { id, accessToken }, { fetchPolicy: 'network-only' });


    onResult((result) => {
      isFavorited.value = result.data.getAdvert.advert.isFavorited;
      console.log(isFavorited);
    });
    const router = useRouter();


    const { mutate: addFavorite } = useMutation(ADD_FAVORITE);
    const { mutate: removeFavorite } = useMutation(REMOVE_FAVORITE);
    const { mutate: deleteAdvert } = useMutation(DELETE_ADVERT);

    const deleteAdvertDB = async function() {
      await deleteAdvert({
        accessToken,
        advertId: id
      }).then(({ data, loading, error }) => {
        if (error) {
          console.error(`An error occurred: ${error.message}`);
          return;
        }

        console.log(data);
      
      });
    }

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

        
        
        });
      

        isFavorited.value = false;
    }

    return {
      result,
      loading,
      error,
      getDate,
      addToFavorite,
      removeFromFavorite,
      deleteAdvertDB,
      isFavorited,
      logedIn
    };

     

  },
});
</script>
