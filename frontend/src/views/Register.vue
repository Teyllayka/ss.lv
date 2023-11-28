<script>
import { ref } from "vue";
import { useMutation } from '@vue/apollo-composable';
import { REGISTER } from "@/graphql/user";
import { useTheme } from "vuetify";
import useVuelidate from '@vuelidate/core'
import { required, email, minLength, helpers } from '@vuelidate/validators'
import { useRouter } from 'vue-router';


const { withMessage, regex } = helpers

export default {
  setup() {
    const { mutate: registerMutation } = useMutation(REGISTER);


    const router = useRouter();
    const theme = useTheme();
    const darkMode = ref(localStorage.getItem("darkMode") === "true" || false);

    const toggleTheme = () => {
      theme.global.name.value = darkMode.value ? "customDarkTheme" : "customLightTheme";
      localStorage.setItem("darkMode", darkMode.value);
    };

    const register = async function() {
      await registerMutation({
        name: this.v$.form.name.$model,
        surname: this.v$.form.surname.$model,
        email: this.v$.form.email.$model,
        phone: this.v$.form.phone.$model,
        password: this.v$.form.password.$model,
      }).then(({ data, loading, error }) => {
        if (error) {
          console.error(`An error occurred: ${error.message}`);
          return;
        }
        console.log(data);
        router.push("/login")
      });
    }

    return { register, toggleTheme, darkMode, v$: useVuelidate() };
  },
  data() {
    return {
      form: {
        email: '',
        password: '',
        name: '',
        surname: '',
        phone: '',
        repeated_password: '',
      },
    }
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
         //  phonePattern: withMessage('Phone must be valid', phone),
        }
      },
    }
  },
}
</script>

<template>
   <section class="reigster text-text">
     
     <div class="background"></div>
     <div class="right">
      
      <div class="info">
         <v-switch
      inset
      color="text"
      v-model="darkMode"
      @change="toggleTheme()"
    ></v-switch>
         <div class="messages">
            <h1>Hello!</h1>
            <h4>Sign Up to Get Started</h4>
         </div>
         <div class="fields">
            <div class="input-field">
               <svg width="26" height="18" viewBox="0 0 24 24" fill="inherit" xmlns="http://www.w3.org/2000/svg">
                  <path d="M16 7C16 9.20914 14.2091 11 12 11C9.79086 11 8 9.20914 8 7C8 4.79086 9.79086 3 12 3C14.2091 3 16 4.79086 16 7Z" stroke="inherit" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                  <path d="M12 14C8.13401 14 5 17.134 5 21H19C19 17.134 15.866 14 12 14Z" stroke="inherit" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
               </svg>
               <input type="text" name="" id="name" v-model="v$.form.name.$model" placeholder="Name"></div>
            <div class="input-field">
               <svg width="26" height="18" viewBox="0 0 24 24" fill="inherit" xmlns="http://www.w3.org/2000/svg">
                  <path d="M16 7C16 9.20914 14.2091 11 12 11C9.79086 11 8 9.20914 8 7C8 4.79086 9.79086 3 12 3C14.2091 3 16 4.79086 16 7Z" stroke="inherit" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                  <path d="M12 14C8.13401 14 5 17.134 5 21H19C19 17.134 15.866 14 12 14Z" stroke="inherit" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
               </svg>
               <input type="text" name="" id="surname" v-model="v$.form.surname.$model" placeholder="Surname"></div>
            <div class="input-field">
               <svg width="26" height="20" viewBox="0 0 24 24" fill="inherit" xmlns="http://www.w3.org/2000/svg">
                  <path d="M16.1007 13.359L15.5719 12.8272H15.5719L16.1007 13.359ZM16.5562 12.9062L17.085 13.438H17.085L16.5562 12.9062ZM18.9728 12.5894L18.6146 13.2483L18.9728 12.5894ZM20.8833 13.628L20.5251 14.2869L20.8833 13.628ZM21.4217 16.883L21.9505 17.4148L21.4217 16.883ZM20.0011 18.2954L19.4723 17.7636L20.0011 18.2954ZM18.6763 18.9651L18.7459 19.7119H18.7459L18.6763 18.9651ZM8.81536 14.7266L9.34418 14.1947L8.81536 14.7266ZM4.00289 5.74561L3.2541 5.78816L3.2541 5.78816L4.00289 5.74561ZM10.4775 7.19738L11.0063 7.72922H11.0063L10.4775 7.19738ZM10.6342 4.54348L11.2346 4.09401L10.6342 4.54348ZM9.37326 2.85908L8.77286 3.30855V3.30855L9.37326 2.85908ZM6.26145 2.57483L6.79027 3.10667H6.79027L6.26145 2.57483ZM4.69185 4.13552L4.16303 3.60368H4.16303L4.69185 4.13552ZM12.0631 11.4972L12.5919 10.9654L12.0631 11.4972ZM16.6295 13.8909L17.085 13.438L16.0273 12.3743L15.5719 12.8272L16.6295 13.8909ZM18.6146 13.2483L20.5251 14.2869L21.2415 12.9691L19.331 11.9305L18.6146 13.2483ZM20.8929 16.3511L19.4723 17.7636L20.5299 18.8273L21.9505 17.4148L20.8929 16.3511ZM18.6067 18.2184C17.1568 18.3535 13.4056 18.2331 9.34418 14.1947L8.28654 15.2584C12.7186 19.6653 16.9369 19.8805 18.7459 19.7119L18.6067 18.2184ZM9.34418 14.1947C5.4728 10.3453 4.83151 7.10765 4.75168 5.70305L3.2541 5.78816C3.35456 7.55599 4.14863 11.144 8.28654 15.2584L9.34418 14.1947ZM10.7195 8.01441L11.0063 7.72922L9.9487 6.66555L9.66189 6.95073L10.7195 8.01441ZM11.2346 4.09401L9.97365 2.40961L8.77286 3.30855L10.0338 4.99296L11.2346 4.09401ZM5.73263 2.04299L4.16303 3.60368L5.22067 4.66736L6.79027 3.10667L5.73263 2.04299ZM10.1907 7.48257C9.66189 6.95073 9.66117 6.95144 9.66045 6.95216C9.66021 6.9524 9.65949 6.95313 9.659 6.95362C9.65802 6.95461 9.65702 6.95561 9.65601 6.95664C9.65398 6.95871 9.65188 6.96086 9.64972 6.9631C9.64539 6.96759 9.64081 6.97245 9.63599 6.97769C9.62634 6.98816 9.61575 7.00014 9.60441 7.01367C9.58174 7.04072 9.55605 7.07403 9.52905 7.11388C9.47492 7.19377 9.41594 7.2994 9.36589 7.43224C9.26376 7.70329 9.20901 8.0606 9.27765 8.50305C9.41189 9.36833 10.0078 10.5113 11.5343 12.0291L12.5919 10.9654C11.1634 9.54499 10.8231 8.68059 10.7599 8.27309C10.7298 8.07916 10.761 7.98371 10.7696 7.96111C10.7748 7.94713 10.7773 7.9457 10.7709 7.95525C10.7677 7.95992 10.7624 7.96723 10.7541 7.97708C10.75 7.98201 10.7451 7.98759 10.7394 7.99381C10.7365 7.99692 10.7335 8.00019 10.7301 8.00362C10.7285 8.00534 10.7268 8.00709 10.725 8.00889C10.7241 8.00979 10.7232 8.0107 10.7223 8.01162C10.7219 8.01208 10.7212 8.01278 10.7209 8.01301C10.7202 8.01371 10.7195 8.01441 10.1907 7.48257ZM11.5343 12.0291C13.0613 13.5474 14.2096 14.1383 15.0763 14.2713C15.5192 14.3392 15.8763 14.285 16.1472 14.1841C16.28 14.1346 16.3858 14.0763 16.4658 14.0227C16.5058 13.9959 16.5392 13.9704 16.5663 13.9479C16.5799 13.9367 16.5919 13.9262 16.6024 13.9166C16.6077 13.9118 16.6126 13.9073 16.6171 13.903C16.6194 13.9008 16.6215 13.8987 16.6236 13.8967C16.6246 13.8957 16.6256 13.8947 16.6266 13.8937C16.6271 13.8932 16.6279 13.8925 16.6281 13.8923C16.6288 13.8916 16.6295 13.8909 16.1007 13.359C15.5719 12.8272 15.5726 12.8265 15.5733 12.8258C15.5735 12.8256 15.5742 12.8249 15.5747 12.8244C15.5756 12.8235 15.5765 12.8226 15.5774 12.8217C15.5793 12.82 15.581 12.8183 15.5827 12.8166C15.5862 12.8133 15.5895 12.8103 15.5926 12.8074C15.5988 12.8018 15.6044 12.7969 15.6094 12.7929C15.6192 12.7847 15.6265 12.7795 15.631 12.7764C15.6403 12.7702 15.6384 12.773 15.6236 12.7785C15.5991 12.7876 15.501 12.8189 15.3038 12.7886C14.8905 12.7253 14.02 12.3853 12.5919 10.9654L11.5343 12.0291ZM9.97365 2.40961C8.95434 1.04802 6.94996 0.83257 5.73263 2.04299L6.79027 3.10667C7.32195 2.578 8.26623 2.63181 8.77286 3.30855L9.97365 2.40961ZM4.75168 5.70305C4.73201 5.35694 4.89075 4.9954 5.22067 4.66736L4.16303 3.60368C3.62571 4.13795 3.20329 4.89425 3.2541 5.78816L4.75168 5.70305ZM19.4723 17.7636C19.1975 18.0369 18.9029 18.1908 18.6067 18.2184L18.7459 19.7119C19.4805 19.6434 20.0824 19.2723 20.5299 18.8273L19.4723 17.7636ZM11.0063 7.72922C11.9908 6.7503 12.064 5.2019 11.2346 4.09401L10.0338 4.99295C10.4373 5.53193 10.3773 6.23938 9.9487 6.66555L11.0063 7.72922ZM20.5251 14.2869C21.3429 14.7315 21.4703 15.7769 20.8929 16.3511L21.9505 17.4148C23.2908 16.0821 22.8775 13.8584 21.2415 12.9691L20.5251 14.2869ZM17.085 13.438C17.469 13.0562 18.0871 12.9616 18.6146 13.2483L19.331 11.9305C18.2474 11.3414 16.9026 11.5041 16.0273 12.3743L17.085 13.438Z" fill="inherit"/>
               </svg>
               <input type="tel" name="" id="phone" v-model="v$.form.phone.$model" placeholder="Phone Number"></div>
            <div class="input-field">
               <svg width="22" height="16" viewBox="0 0 22 16" fill="inherit" xmlns="http://www.w3.org/2000/svg">
                  <path fill-rule="evenodd" clip-rule="evenodd" d="M0.5 1.25L1.25 0.5H20.75L21.5 1.25V14.75L20.75 15.5H1.25L0.5 14.75V1.25ZM2 2.8025V14H20V2.804L11.465 9.35H10.55L2 2.8025ZM18.545 2H3.455L11 7.8035L18.545 2Z" fill="inherit"/>
               </svg>
               <input type="email" name="" id="email" v-model="v$.form.email.$model" placeholder="Email Address"></div>
            <div class="input-field">
               <svg id="password" width="16" height="20" viewBox="0 0 16 20" fill="inherit" xmlns="http://www.w3.org/2000/svg">
                  <path d="M16 10C16 8.897 15.103 8 14 8H13V5C13 2.243 10.757 0 8 0C5.243 0 3 2.243 3 5V8H2C0.897 8 0 8.897 0 10V18C0 19.103 0.897 20 2 20H14C15.103 20 16 19.103 16 18V10ZM5 5C5 3.346 6.346 2 8 2C9.654 2 11 3.346 11 5V8H5V5Z" fill="inherit"/>
               </svg>
               <input type="password" name="" id="password" v-model="v$.form.password.$model" placeholder="Password"></div>
               <div class="errors">
               <div class="input-errors" v-for="(error, index) of v$.form.email.$errors" :key="index">
                  <div class="error-msg">{{ error.$message }}</div>
               </div>
               <div class="input-errors" v-for="(error, index) of v$.form.phone.$errors" :key="index">
                  <div class="error-msg">{{ error.$message }}</div>
               </div>
               <div class="input-errors" v-for="(error, index) of v$.form.name.$errors" :key="index">
                  <div class="error-msg">{{ error.$message }}</div>
               </div>
               <div class="input-errors" v-for="(error, index) of v$.form.surname.$errors" :key="index">
                  <div class="error-msg">{{ error.$message }}</div>
               </div>
               <div class="input-errors" v-for="(error, index) of v$.form.password.$errors" :key="index">
                  <div class="error-msg">{{ error.$message }}</div>
               </div>
            </div>
            <button class="press" :disabled="v$.form.$invalid" @click="register">Register</button>
            <div class="urls">
               <router-link to="/login">Login Instead</router-link>
               <router-link to="/home">Not Now</router-link>
            </div>
            
         </div>
      </div>
     </div>
   </section>
</template>


<style scoped>


.errors {
   font-size:14px;
   display: flex;
   justify-content: center;
   align-items: center;
   flex-direction: column;
   gap:5px;
}

#password {
   margin-left:6px;
}

.background {
   width: 70%;
   height: 100%;
   background-image: linear-gradient(to top, rgb(var(--v-theme-bg1)), rgb(var(--v-theme-bg2)), rgb(var(--v-theme-bg3)), rgb(var(--v-theme-bg4)), rgb(var(--v-theme-text)));
}

h1 {
   margin-bottom: 10px;
   text-align: start;
}

h4 {
   text-align: start;
}

section {
   height: 100%;
   width: 100%;
   display: flex;
   justify-content: center;
   align-items: center;
}

.info {
   width: 50%;
   display: flex;
   justify-content: center;
   align-items: flex-start;
   flex-direction: column;
   gap:40px;
}

.right {
   min-width: 576px;
   width: 30%;
   display: flex;
   justify-content: center;
   align-items: center;
  
}

@media only screen and (max-width: 576px) {
   .background {
   display: none;
   }
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
   padding: 10px 0;
   border-radius: 50px;
   background-color: transparent;
   border: 1px solid  rgba(var(--v-theme-inputText), 0.3);
   display: flex;
   justify-content: center;
   align-items: center;
   flex-direction: row;
   gap: 5px;
}

.input-field svg {
   fill: rgba(var(--v-theme-inputText), 0.3);
}

input {
   color: rgba(var(--v-theme-inputText), 0.87);
   outline: none;

}

input::placeholder {
   color: rgba(var(--v-theme-inputText), 0.3);

}

.urls {
   width: 100%;
   display: flex;
   justify-content: space-around;
   align-items: center;
   flex-direction: row;
}



form {
   display: flex;
   justify-content: center;
   align-items: center;
   flex-direction: column;
}


</style>
