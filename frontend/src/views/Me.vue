
<template>
  
   <profile v-if="logedIn" />

</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { ME } from "@/graphql/user";
import { useQuery } from '@vue/apollo-composable'
import Profile from '../components/Profile.vue';
import Register from '../components/Register.vue';

export default defineComponent({
  name: 'App',
  setup() {
      let access_token = localStorage.getItem("access_token");
      if (access_token !== undefined) {
        console.log(access_token)
        const { result, loading, error } = useQuery(ME, { access_token });

        console.log(result.value);


        if (result.value === undefined) {
          console.log("undefined");
          return {
            logedIn: false,
          }
        } 
        return {
          result,
          error,
        };
      } else {
        return {
          logedIn: false,
        }
      }

      
    },
  components: {
    Profile,
  },
});
</script>


