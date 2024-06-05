<template>
   <div v-if="loading"><loading></loading></div>
   <div v-else-if="error"><error v-bind="error"/></div>
   <div class="edit-profile" v-else>
      <div class="title" v-if="!error"> <h1>Edit Profile:</h1></div>
      <section class="data">
    <div class="input-field">
        <label for="name">Name:</label>
        <input class="name" v-model="v$.form.name.$model">
        <div class="input-errors" v-for="(error, index) of v$.form.name.$errors" :key="index">
            <div class="error-msg">{{ error.$message }}</div>
        </div>
    </div>

    <!-- Surname Input -->
    <div class="input-field">
        <label for="surname">Surname:</label>
        <input class="surname" v-model="v$.form.surname.$model">
        <div class="input-errors" v-for="(error, index) of v$.form.surname.$errors" :key="index">
            <div class="error-msg">{{ error.$message }}</div>
        </div>
    </div>

    <!-- Email Input -->
    <div class="input-field">
        <label for="email">Email:</label>
        <input class="email" v-model="v$.form.email.$model">
        <div class="input-errors" v-for="(error, index) of v$.form.email.$errors" :key="index">
            <div class="error-msg">{{ error.$message }}</div>
        </div>
    </div>

    <!-- Phone Input -->
    <div class="input-field">
        <label for="phone">Phone:</label>
        <input class="phone" v-model="v$.form.phone.$model">
        <div class="input-errors" v-for="(error, index) of v$.form.phone.$errors" :key="index">
            <div class="error-msg">{{ error.$message }}</div>
        </div>
    </div>

    <!-- Password Input -->
    <div class="input-field">
        <label for="password">Your Password:</label>
        <input class="password" type="password" v-model="v$.form.password.$model">
        <div class="input-errors" v-for="(error, index) of v$.form.password.$errors" :key="index">
            <div class="error-msg">{{ error.$message }}</div>
        </div>
    </div>

    <div class="input-field">
      <label for="password">Image :</label>
               <input type="file" accept="image/*" name="" id="image" @change="handleImageUpload" placeholder="Image">
              
               
               
          
            </div>
    
    <button class="press" @click="edit">Edit</button>
    <div v-if="qError" class="apError">Wrong Password!</div>
</section>


    </div>
</template>

<style scoped>

.apError {
  color: red;
  font-size:24px;
}


@media (max-width: 1400px) {
  .title, .data, .title {
      margin: 50px 20px !important;
  }
  
}

@media (max-width: 425px) {
  .data {
    justify-content: center !important;
    align-items: center !important;
    text-align: center !important;
    gap:20px;
  }

  .press {
    width: 100%;
  }

  .input-field {
    display: flex !important;
    justify-content: center !important;
    align-items: center !important;
    width: 100% !important;
    flex-direction: column !important;
    gap:10px;
  
  }

  input {
    text-align: center;
  }

 
  
}

.data, .title {
  margin: 50px 150px;
}
h1 {
  color: rgb(var(--v-theme-text));

}

label {
  color: rgb(var(--v-theme-text));
  font-size:24px;
}

.press {
   width: 200px;
   padding: 10px 0;
   border-radius: 50px;
   color: rgb(var(--v-theme-background));
   margin-top:50px;
  }
input {
  font-size:18px;
}

.data {
  display: flex;
  justify-content: flex-start;
  align-items: flex-start;
  flex-direction: column;
}

.input-field {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: row;
  gap:20px;
}

.error-msg {
  color: rgb(var(--v-theme-text));
}

</style>


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
   const qError = ref(null);
   const { result, loading, error } = useQuery(ME, { accessToken });

   const { mutate: editMutation } = useMutation(EDIT);
   const v$ = useVuelidate();
   const router = useRouter();

   const selectedFile = ref(null);

   const handleImageUpload = (event) => {
      selectedFile.value = event.target.files[0];
      };



   const edit = async function() {
      if (this.v$.form.$invalid) {
         console.log(this.v$.form);

      } else {
        const formData = new FormData();
         formData.append("file", selectedFile.value);
         const response = await fetch('https://gachi.gay/api/upload', {
            method: 'POST',
            body: formData,
         });
         const data = await response.json();
         
         await editMutation({
            name: this.v$.form.name.$model,
            surname: this.v$.form.surname.$model,
            email: this.v$.form.email.$model,
            phone: this.v$.form.phone.$model,
            password: this.v$.form.password.$model,
            avatarUrl: data.error == 400 ? "" : data.link,
            accessToken,
         }).then(({ data }) => {

            console.log(data);
            router.push("/")
         }).catch((err) => {
            console.log("err", err);
            qError.value = err;
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
     v$,
     handleImageUpload,
     qError
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
          min: withMessage('Password must be at least 6 characters', minLength(6))
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
          regex: withMessage('Phone number must match the format +12300000000', value => /^\+\d{11}$/.test(value)),
        }
      },
    }
  },
});
</script>