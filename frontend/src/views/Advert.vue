<template>
  <p v-if="error">Something went wrong... {{ error.message }}</p>
  <p v-if="loading">Loading...</p>
  <section class="advert" v-else>
    <h1 class="title"> {{ result.getAdvert.advert.category  }}</h1>
   
    <div class="up">
      <div class="image-container">
        <img src="https://30.img.avito.st/image/1/1.zjCnc7a4YtmR2qDct2SnJbbRYN8Z0uDR0ddg2xfaatMR.xajaZfbTIv0ViaQVJN-mahjP5knob4APgvyLH6pQDCU" width="300" height="300" alt="">
      </div>
      <div class="info">
        <h1 class="title"> {{ result.getAdvert.advert.title }}</h1>
        <p class="price"> {{ result.getAdvert.advert.price }} €</p>
       
        <p class="posted">posted {{ getDate(result.getAdvert.advert.createdAt) }} by <router-link :to="'/user/' + result.getAdvert.user.id" class="link">{{ result.getAdvert.user.name }} {{ result.getAdvert.user.surname }}</router-link></p>
        <!-- <p>you can contact him by phone <router-link :to="'tel:' + result.getAdvert.user.phone">{{ result.getAdvert.user.phone }}</router-link> or by email <router-link :to="'mailto:' + result.getAdvert.user.email"></router-link>{{ result.getAdvert.user.email }}</p> -->
        <p>you can contact him by phone {{ result.getAdvert.user.phone }} or by email {{ result.getAdvert.user.email }}</p>

        <div class="message">
          <button>Message</button>
        </div>
      </div>
    </div>
    <div class="down">
      
      {{ result.getAdvert.advert.description }}


      <p>Characteristics:</p>
    </div>
    <!-- {{ result }} -->
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

.info .message {
  margin-top:100px;
  width:100%;
  display: flex;
  justify-content: center;
  align-items: center;
}

.message button {
   
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
import { defineComponent } from 'vue';
import { useQuery } from '@vue/apollo-composable'
import { GET_ADVERT } from "@/graphql/advert";
import { useRoute } from 'vue-router';


export default defineComponent({
  name: 'App',
  setup() {

    const route = useRoute()

    let id = parseInt(route.params.id[0]);


    const { result, loading, error } = useQuery(GET_ADVERT, {id});

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

   

    return {
      result,
      loading,
      error,
      getDate
    };
  },
});
</script>
