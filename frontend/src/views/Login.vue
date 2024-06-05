
<script>
  import { useMutation } from '@vue/apollo-composable';
  import { ref, watchEffect } from "vue";
  import { useTheme } from "vuetify";
  import { LOGIN } from "@/graphql/user";
  import useVuelidate from '@vuelidate/core'
  import { required,  helpers, email } from '@vuelidate/validators'
  const { withMessage } = helpers
  import { useRouter } from 'vue-router';


  
  export default {
    setup() {
     
      const router = useRouter();
      const { mutate: loginMutation } = useMutation(LOGIN);
      const qError = ref(null);

  
    
      const theme = useTheme();
      const darkMode = ref(localStorage.getItem("darkMode") === "true" || false);

      const toggleTheme = () => {
         theme.global.name.value = darkMode.value ? "customDarkTheme" : "customLightTheme";
         localStorage.setItem("darkMode", darkMode.value);
      };

      watchEffect(() => {
         localStorage.setItem("darkMode", darkMode.value);
      });

      const login = async function() {
         console.log(this.v$)
        await loginMutation({
          email: this.v$.form.email.$model,
          password: this.v$.form.password.$model,
          
        }).then(({ data }) => {
          
          console.log(data.login.accessToken);
          localStorage.setItem("access_token", data.login.accessToken);
          console.log(localStorage.getItem("access_token"))
          localStorage.setItem("refresh_token", data.login.refreshToken);
          localStorage.setItem("logedIn", "true");
          console.log(data);
          router.push("/home")
         }).catch((err) => {
            console.log("err", err);
            qError.value = err;

         
         });
  
      }

    
  
      return { login, toggleTheme, darkMode, v$: useVuelidate(), qError };
    },
    data() {
      return {
         form: {
            email: '',
            password: '',
         },
      }
   },
   validations() {
    return {
      form: {
        email: {
          required: withMessage('Email is required', required),
          email: withMessage('Email must be valid', email),
        },
        password: {
          required: withMessage('Password is required', required),
        },
      },
    }
  },
  }
  

</script>


<template>
   <section class="reigster text-text">
     <div class="right">
      <div class="info">
      
         <div class="messages">
            <h1>Hello Again!</h1>
            <span>Welcome Back</span>
         </div>
         <div class="fields">
            <div class="input-field">
               <svg width="22" height="16" viewBox="0 0 22 16" fill="inherit" xmlns="http://www.w3.org/2000/svg">
                  <path fill-rule="evenodd" clip-rule="evenodd" d="M0.5 1.25L1.25 0.5H20.75L21.5 1.25V14.75L20.75 15.5H1.25L0.5 14.75V1.25ZM2 2.8025V14H20V2.804L11.465 9.35H10.55L2 2.8025ZM18.545 2H3.455L11 7.8035L18.545 2Z" fill="inherit"/>
               </svg>
               <input type="email" name="" id="email" v-model="v$.form.email.$model" placeholder="Email Address"></div>
            <div class="input-field" >
               <svg id="password" width="16" height="20" viewBox="0 0 16 20" fill="inherit" xmlns="http://www.w3.org/2000/svg">
                  <path d="M16 10C16 8.897 15.103 8 14 8H13V5C13 2.243 10.757 0 8 0C5.243 0 3 2.243 3 5V8H2C0.897 8 0 8.897 0 10V18C0 19.103 0.897 20 2 20H14C15.103 20 16 19.103 16 18V10ZM5 5C5 3.346 6.346 2 8 2C9.654 2 11 3.346 11 5V8H5V5Z" fill="inherit"/>
               </svg>
               <input type="password" name="" id="password" v-model="v$.form.password.$model" placeholder="Password"></div>
            <div class="errors">
               <div class="input-errors" v-for="(error, index) of v$.form.email.$errors" :key="index">
                  <div class="error-msg">{{ error.$message }}</div>
               </div>
               <div class="input-errors" v-for="(error, index) of v$.form.password.$errors" :key="index">
                  <div class="error-msg">{{ error.$message }}</div>
               </div>
            </div>
            <button class="press" :disabled="v$.form.$invalid" @click="login">Login</button>
            <div v-if="qError" class="apError">Wrong Email Or Password!</div>
            <div class="urls">
               <router-link to="/register">Register Instead</router-link>
               <router-link to="/home">Not Now</router-link>
            </div>
           
         </div>
     </div>
     </div>
   </section>
</template>


<style scoped>


.apError {
  color:  rgb(var(--v-theme-text));
  font-size:24px;
}


.messages {
   display: flex;
   justify-content: center;
   align-items: center;
   flex-direction: column;
   width:100%;

   


   h1 {
      text-align: start;
      font-size:32px;
      
   }

   h4 {
      text-align: start;
   }
   
}

#password {
   margin-left:6px;
}



section {
   height: 100%;
   width: 100%;
   display: flex;
   justify-content: center;
   align-items: center;
}

@media (max-width: 1000px) {
   .info {
      width:90% !important;
   }

}

.info {
   width: 50%;
   display: flex;
   justify-content: center;
   align-items: flex-start;
   flex-direction: column;
   gap:20px;
}

.right {
   
   width: 50%;
   display: flex;
   justify-content: center;
   align-items: center;
  
}

@media only screen and (max-width: 576px) {

   .right {
      width: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
   
   }
}

.fields {
   width: 100%;
   display: flex;
   justify-content: center;
   align-items: center;
   flex-direction: column;
   gap: 20px;
}


.press {
   width: 100%;
   padding: 10px 0;
   border-radius: 50px;
   color: rgb(var(--v-theme-background));
}

.input-field {
   width: 100%;
   padding: 10px 20px;
   border-radius: 50px;
   background-color: transparent;
   border: 1px solid  rgba(var(--v-theme-inputText), 0.3);
   display: flex;
   align-items: center;
   flex-direction: row;
   gap: 5px;
}

.input-field svg {
   fill: rgba(var(--v-theme-inputText), 0.3);
   flex-shrink: 0;
}

input {
   color: rgba(var(--v-theme-inputText), 0.87);
   outline: none;
   width:100%;
}

input::placeholder {
   color: rgba(var(--v-theme-inputText), 0.3);

}

.urls {
   width: 100%;
   display: flex;
   justify-content: space-between;
   align-items: center;
   flex-direction: row;
}

form {
   display: flex;
   justify-content: center;
   align-items: center;
   flex-direction: column;
}
.register-button {
  transition: background-color 0.5s ease, color 0.2s ease;
  background-color: rgb(var(--v-theme-text));
  color: rgb(var(--v-theme-background));
}

.register-button:hover {
  background-color: rgb(var(--v-theme-text_highlight));
  
}
</style>

