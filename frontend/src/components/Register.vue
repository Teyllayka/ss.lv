<template>
    <div id="register" class="text-text">
        <form v-if="isRegistering" @submit.prevent="register">
            <input v-model="name" type="text" placeholder="Name" required />
            <input v-model="surname" type="text" placeholder="Surname" required />
            <input v-model="email" type="email" placeholder="Email" required />
            <input v-model="phone" type="text" placeholder="Phone" required />
            <input v-model="password" type="password" placeholder="Password" required />
            <button type="submit">Register</button>
        </form>
        <form v-else @submit.prevent="login">
            <input v-model="email" type="email" placeholder="Email" required />
            <input v-model="password" type="password" placeholder="Password" required />
            <button type="submit">Login</button>
        </form>
        <button @click="toggleForm">
            {{ isRegistering ? 'Switch to Login' : 'Switch to Register' }}
        </button>
    </div>
</template>
  
<script>
  import { useMutation } from '@vue/apollo-composable';
  import { ref } from 'vue';
  import { REGISTER } from "@/graphql/user";
  
  export default {
    setup() {
      const name = ref('');
      const surname = ref('');
      const email = ref('');
      const phone = ref('');
      const password = ref('');
      const isRegistering = ref(true); // Added
      
      const { mutate: registerMutation } = useMutation(REGISTER);
  
      const register = async function() {
        console.log("registered");
  
  
        console.log(this.email);
        console.log(this.name);
        console.log(this.surname);
        const { result, loading, error } = await registerMutation({
            name: this.name,
            surname: this.surname,
            email: this.email,
            phone: this.phone,
            password: this.password,
          
        });
  
        if (error) {
          console.error(`An error occurred: ${error.message}`);
          return;
        }
  
        console.log(result);
      }

      const toggleForm = function() { // Added
        isRegistering.value = !isRegistering.value;
      }
  
      return { register, name, surname, email, phone, password, isRegistering, toggleForm };
    }
  }
</script>

<!-- 
<template>
    <div id="auth">
      <form v-if="isRegistering" @submit.prevent="register">
        <input v-model="name" type="text" placeholder="Name" required />
        <input v-model="surname" type="text" placeholder="Surname" required />
        <input v-model="email" type="email" placeholder="Email" required />
        <input v-model="phone" type="text" placeholder="Phone" required />
        <input v-model="password" type="password" placeholder="Password" required />
        <button type="submit">Register</button>
      </form>
      <form v-else @submit.prevent="login">
        <input v-model="email" type="email" placeholder="Email" required />
        <input v-model="password" type="password" placeholder="Password" required />
        <button type="submit">Login</button>
      </form>
      <button @click="toggleForm">
        {{ isRegistering ? 'Switch to Login' : 'Switch to Register' }}
      </button>
    </div>
  </template>
  
  <script>
    import { useMutation } from '@vue/apollo-composable';
    import { ref } from 'vue';
    import { REGISTER } from "@/graphql/user";
    
    export default {
      setup() {
        const name = ref('');
        const surname = ref('');
        const email = ref('');
        const phone = ref('');
        const password = ref('');
        const isRegistering = ref(true); // Added
  
        const { mutate: registerMutation } = useMutation(REGISTER);
        const { mutate: loginMutation } = useMutation(LOGIN); // Added
  
        const register = async function() {
          const { result, loading, error } = await registerMutation({
              name: this.name,
              surname: this.surname,
              email: this.email,
              phone: this.phone,
              password: this.password,
          });
    
          if (error) {
            console.error(`An error occurred: ${error.message}`);
            return;
          }
    
          console.log(result);
        }
  
        const login = async function() { // Added
        //   const { result, loading, error } = await loginMutation({
        //       email: this.email,
        //       password: this.password,
        //   });
    
        //   if (error) {
        //     console.error(`An error occurred: ${error.message}`);
        //     return;
        //   }
    
        //   console.log(result);
        }
  
        const toggleForm = function() { // Added
          isRegistering.value = !isRegistering.value;
        }
    
        return { register, login, toggleForm, isRegistering, name, surname, email, phone, password };
      }
    }
  </script>
   -->