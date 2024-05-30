<template>
   <div v-if="loading"><loading></loading></div>
   <div v-else-if="error"><error v-bind="error"/></div>
   <section class="edit-profile" v-else>
      <div class="data">
         <input class="name" v-model="v$.form.name.$model" >

         <div class="input-errors" v-for="(error, index) of v$.form.name.$errors" :key="index">
            <div class="error-msg">{{ error.$message }}</div>
         </div>

         <input class="surname" v-model="v$.form.surname.$model" >

         <div class="input-errors" v-for="(error, index) of v$.form.surname.$errors" :key="index">
            <div class="error-msg">{{ error.$message }}</div>
         </div>

         <input class="email" v-model="v$.form.email.$model" >

         <div class="input-errors" v-for="(error, index) of v$.form.email.$errors" :key="index">
            <div class="error-msg">{{ error.$message }}</div>
         </div>

         <input class="phone" v-model="v$.form.phone.$model" >

         <div class="input-errors" v-for="(error, index) of v$.form.phone.$errors" :key="index">
            <div class="error-msg">{{ error.$message }}</div>
         </div>


         <input class="password" v-model="v$.form.password.$model" >
      </div>

      <button class="press" @click="edit" >edit</button>
    </section>
</template>


<script >
import { defineComponent, ref, watch } from 'vue';
import { EDIT, ME } from "@/graphql/user";
import { useQuery } from '@vue/apollo-composable'
import Error from '../components/Error.vue';
import Loading from '../components/Loading.vue';
import { useMutation } from '@vue/apollo-composable';
import { required, email, minLength, helpers } from '@vuelidate/validators'
import useVuelidate from '@vuelidate/core'
import { useRouter } from 'vue-router';
const { withMessage } = helpers

export default defineComponent({
 name: 'App',
 components: {
   Error,
   Loading,
 },
 setup() {
   const accessToken = localStorage.getItem("access_token");
   const { result, loading, error } = useQuery(ME, { accessToken });

   const { mutate: editMutation } = useMutation(EDIT);
   const v$ = useVuelidate();
   const router = useRouter();


   const edit = async function() {
      if (this.v$.form.$invalid) {
         console.log(this.v$.form);

      } else {
         await editMutation({
            name: this.v$.form.name.$model,
            surname: this.v$.form.surname.$model,
            email: this.v$.form.email.$model,
            phone: this.v$.form.phone.$model,
            password: this.v$.form.password.$model,
            accessToken,
         }).then(({ data, loading, error }) => {
            
            if (error) {
               console.error(`An error occurred: ${error.message}`);
               return;
            }

            console.log(data);
            router.push("/")
         });


      }
   }

   const form = ref({
      name: '',
      surname: '',
      email: '',
      phone: '',
      password: '',
    });

   
    watch(result, (newResult) => {
    if (newResult && newResult.me) { 
      form.value = {
        name: newResult.me.name,
        surname: newResult.me.surname,
        email: newResult.me.email,
        phone: newResult.me.phone,
        password: newResult.me.password, 
      };
    }
  }, { immediate: true });

   return {
     result,
     error,
     form,
     loading,
     edit,
     v$
   };
 },

  validations() {
    return {
      form: {
        email: {
          required: withMessage('Email is required', required),
          email: withMessage('Email must be valid', email),
          min: withMessage('Email must be at least 6 characters', minLength(6))
        },
        password: {
          required: withMessage('Password is required', required),
        }, 
        name: {
          required: withMessage('Name is required', required),
          min: withMessage('Name must be at least 6 characters', minLength(6))
        },
        surname: {
          required: withMessage('Surname is required', required),
          min: withMessage('Surname must be at least 6 characters', minLength(6))
        },
        phone: {
          required: withMessage('Phone number is required', required),
        }
      },
    }
  },
});
</script>