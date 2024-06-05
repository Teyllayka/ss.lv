
<script >
import { defineComponent } from 'vue';
import { ADD_FAVORITE, REMOVE_FAVORITE } from "@/graphql/advert";
import { useMutation } from '@vue/apollo-composable';
import { useRouter } from 'vue-router';

export default defineComponent({
  name: 'App',
  props: ['location', 'price', 'id', 'title', 'createdAt', 'available', 'isFavorited', 'photoUrl'],
  setup(props, { emit }) {  
    const router = useRouter();


    let timestamp = props.createdAt;
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

    const accessToken = localStorage.getItem("access_token");

    const { mutate: addFavorite } = useMutation(ADD_FAVORITE);
    const { mutate: removeFavorite } = useMutation(REMOVE_FAVORITE);


    const addToFavorite = async function() {
      const logedIn = localStorage.getItem('logedIn') === 'true';
      if (logedIn) {
        await addFavorite({
          accessToken,
          advertId: props.id
          
        }).then(({ data, loading, error }) => {
          if (error) {
            console.error(`An error occurred: ${error.message}`);
            return;
          }

          console.log(data);
        
         });
         console.log("emit true");
        
         console.log(props.id)
        emit('update:isFavorited', props.id, true);
  
      } else {
        router.push("/login")

      }
        
    }

    const removeFromFavorite = async function() {
        await removeFavorite({
          accessToken,
          advertId: props.id
          
        }).then(({ data, loading, error }) => {
          if (error) {
            console.error(`An error occurred: ${error.message}`);
            return;
          }

          console.log(data);
        
         });
         console.log(props.id)
         if (this.$route.name == "bookmarks") {
           console.log("bookmarks");
           document.getElementById(props.id).remove();
           
         }

       
         emit('update:isFavorited', props.id, false);
  
    }

    return { formattedDate, addToFavorite, removeFromFavorite };
  },
});

</script>

<template>

  
<div class="advert" :id="id">
  <router-link :to="'/advert/' + id" class="link"> <img :src="photoUrl"  alt="">
  </router-link>
 
  <div class="description">
    <div class="upper">
      <div class="title">{{ title }}</div>
      <div class="details">{{ location }}</div>
      <div class="details">{{ formattedDate }}</div>
    </div>
  

    <div class="downer">
      <div class="price"> {{ price.toFixed(2) }} €</div>
      <button class="addToFavBtn" @click="addToFavorite" v-if="!isFavorited">
        <svg fill="inherit" height="25" width="25" version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" 
          viewBox="0 0 471.701 471.701" xml:space="preserve" stroke-width="10">
          <g>
            <path d="M433.601,67.001c-24.7-24.7-57.4-38.2-92.3-38.2s-67.7,13.6-92.4,38.3l-12.9,12.9l-13.1-13.1
              c-24.7-24.7-57.6-38.4-92.5-38.4c-34.8,0-67.6,13.6-92.2,38.2c-24.7,24.7-38.3,57.5-38.2,92.4c0,34.9,13.7,67.6,38.4,92.3
              l187.8,187.8c2.6,2.6,6.1,4,9.5,4c3.4,0,6.9-1.3,9.5-3.9l188.2-187.5c24.7-24.7,38.3-57.5,38.3-92.4
              C471.801,124.501,458.301,91.701,433.601,67.001z M414.401,232.701l-178.7,178l-178.3-178.3c-19.6-19.6-30.4-45.6-30.4-73.3
              s10.7-53.7,30.3-73.2c19.5-19.5,45.5-30.3,73.1-30.3c27.7,0,53.8,10.8,73.4,30.4l22.6,22.6c5.3,5.3,13.8,5.3,19.1,0l22.4-22.4
              c19.6-19.6,45.7-30.4,73.3-30.4c27.6,0,53.6,10.8,73.2,30.3c19.6,19.6,30.3,45.6,30.3,73.3
              C444.801,187.101,434.001,213.101,414.401,232.701z"/>
          </g>
        </svg>
      </button>
      <button @click="removeFromFavorite" v-else>
        <svg xmlns="http://www.w3.org/2000/svg"  viewBox="0 0 50 50" width="25px" height="25px"><path d="M 9.15625 6.3125 L 6.3125 9.15625 L 22.15625 25 L 6.21875 40.96875 L 9.03125 43.78125 L 25 27.84375 L 40.9375 43.78125 L 43.78125 40.9375 L 27.84375 25 L 43.6875 9.15625 L 40.84375 6.3125 L 25 22.15625 Z"/></svg>
      </button>
      

    </div>
  </div>
</div>
</template>

<style scoped>

.addToFavBtn {
  height: 25px;
}

@media only screen and (max-width: 900px) {

  .upper {
    margin: 4px 0px 0px 8px !important;
  }

  .downer {
    padding: 0px 8px !important;
  }

  .advert {
    width: 90% !important;
    height: 90% !important;
  }

  .title  {
    font-size: 16px !important;
  }

  .details {
    font-size: 12px !important;
  }
}

.advert {
  width: 370px;
  height: 370px;
  border: 1px solid black;
  border-radius: 8px 8px 8px 8px;
  display: flex;
  flex-direction: column;
}

.advert .description {
  display: flex;
  justify-content: center;
  align-items: flex-start;
  flex-direction: column;
  height: 65%;
  max-height: 200px;
  width:100%;
}
.advert .title {
  word-wrap: break-word;
  text-align: left;
  color: rgb(var(--v-theme-text)); 
  font-size: 20px; 
  width: 90%; 
  overflow: hidden; 
  white-space: nowrap; 
  text-overflow: ellipsis; 
}

.advert .details {
  overflow: hidden; 
  white-space: nowrap; 
  text-overflow: ellipsis; 
  width: 90%; 
}


.advert img {
  border-radius: 8px 8px 0 0;
  width:100%;
  height:100px;
  object-fit:cover;
}


.upper {
  margin: 18px 0px 0px 12px;
  height: 60%;
  width:100%;
}

.downer {
  border-top: 1px solid #343741;
  width: 100%;
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-direction: row;
  height: 40%;
  padding: 0 20px;
  
}

.downer button {
  background-color: transparent;
  fill: rgb(var(--v-theme-text));
  stroke: rgb(var(--v-theme-text));
}

.link {
  height:45%;
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
}



.price {
  border-radius: 28px;
  font-size:24px;
  /* border: 1px solid rgb(var(--v-theme-text)); */
  /* box-shadow: 0px 0px 24px 0px rgba(253, 230, 63, 0.40); */
}

.details {
  color: #828282;
  text-align: left;
}

</style>


