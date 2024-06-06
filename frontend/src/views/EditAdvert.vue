<template>
   <div v-if="loading"><loading></loading></div>
   <div v-else-if="error"><error v-bind="error"/></div>
   <div class="edit-profile" v-else>
      <div class="title" v-if="!error"> <h1>Edit Advert:</h1></div>
      <section class="data">
    <div class="input-field">
        <label for="name">Price:</label>
        <input class="name" v-model="v$.form.price.$model">
        <div class="input-errors" v-for="(error, index) of v$.form.price.$errors" :key="index">
            <div class="error-msg">{{ error.$message }}</div>
        </div>
    </div>

    <!-- Surname Input -->
    <div class="input-field">
        <label for="surname">Title:</label>
        <input class="surname" v-model="v$.form.title.$model">
        <div class="input-errors" v-for="(error, index) of v$.form.title.$errors" :key="index">
            <div class="error-msg">{{ error.$message }}</div>
        </div>
    </div>

    <!-- Email Input -->
    <div class="input-field">
        <label for="email">Description:</label>
        <textarea class="email" v-model="v$.form.description.$model" ></textarea>
        <div class="input-errors" v-for="(error, index) of v$.form.description.$errors" :key="index">
            <div class="error-msg">{{ error.$message }}</div>
        </div>
    </div>

    <div class="input-field">
        <label for="email">Location:</label>
        <input class="surname" v-model="v$.form.location.$model">
        <div class="input-errors" v-for="(error, index) of v$.form.location.$errors" :key="index">
            <div class="error-msg">{{ error.$message }}</div>
        </div>
    </div>

    <div class="input-field last">
        <label for="photos">Photos:</label>
        <input type="file" accept="image/*" id="photos" multiple @change="handleFileUpload($event)"  />
      </div>

    <button class="press" @click="edit">Edit</button>
    <div v-if="qError" class="apError">Wrong Password!</div>
</section>


    </div>
</template>


<script>
import { defineComponent, ref, watch } from 'vue';
import { GET_ADVERT, EDIT_ADVERT } from "@/graphql/advert";
import { useQuery } from '@vue/apollo-composable'
import Error from '../components/Error.vue';
import Loading from '../components/Loading.vue';
import { useMutation } from '@vue/apollo-composable';
import { required, email, minLength, helpers } from '@vuelidate/validators'
import useVuelidate from '@vuelidate/core'
import { useRouter } from 'vue-router';
import { useRoute } from 'vue-router';

const { withMessage } = helpers

export default defineComponent({
  name: 'App',
  components: {
    Error,
    Loading
  },
  setup() {
    const router = useRouter();
      const route = useRoute();
    let id = parseInt(route.params.id);
    const accessToken = localStorage.getItem("access_token");
    const { result, loading, error } = useQuery(GET_ADVERT, { id, accessToken  }, { fetchPolicy: 'network-only' });
    const { mutate: editMutation } = useMutation(EDIT_ADVERT);
    const qError = ref(null);
    const v$ = useVuelidate()
    const selectedFile = ref([]);

    const handleFileUpload = (event) => {
  console.log("files", event.target.files);
  selectedFile.value = event.target.files; // Corrected variable name here
};


    const edit = async function() {
      //let formToSend = {};


      // formKeys.forEach(key => {
      //    console.log(key);

      //    if ( key === 'category' || key === 'price' || key === 'location' || key === 'title' || key === 'description' || key === 'photoUrl') {
      //    return;
      //    }

      //    try {
      //    formToSend[key] = this.v$.form[key].$model;
      //    } catch (error) {
      //    // do nothing
         
      //    }
      // });

      let urls = [];
        
        for (let i = 0; i < selectedFile.value.length; i++) {
          const formData = new FormData();
          formData.append('file', selectedFile.value[i]);
          const response = await fetch('https://gachi.gay/api/upload', {
            method: 'POST',
            body: formData,
         });
         const data = await response.json();
          urls.push(data.link);
        }

        if (urls.length == 0) {
          urls.push('');
        }

      await editMutation({
        id: id,
         price: parseFloat(this.v$.form.price.$model),
          location: this.v$.form.location.$model,
          title: this.v$.form.title.$model,
          description: this.v$.form.description.$model,
          //data: formToSend,
          photos: urls,
          accessToken
      }).then(({ data }) => {
        console.log(data);
        router.push("/home")
      }).catch((err) => {
        console.log("err", err);
        qError.value = err;
      });
    }

    const form = ref({
      title: '',
      description: '',
      price: '',
      location: '',

    });

   
    watch(result, (newResult) => {
      if (newResult && newResult.getAdvert) { 
         form.value = {
            title: newResult.getAdvert.advert.title,
            description: newResult.getAdvert.advert.description,
            price: newResult.getAdvert.advert.price,
            location: newResult.getAdvert.advert.location,
         };

         console.log(form.value);
      }
   }, { immediate: true });

    return {
      result,
      error,
      loading,
      form,
      v$,
      edit,
      qError,
      handleFileUpload
    };
  },
  validations() {
    return {
      form: {
        price: {
          required: withMessage('Price is required', required),
          regex: withMessage('Price must be a number', (value) => /^(?!0)[1-9]\d{0,4}$/
          .test(value)),
        },
        location: {required: withMessage('Surname is required', required)},
        title: {required: withMessage('title is required', required)},
        description: {required: withMessage('description is required', required)},
        //category: {required: withMessage('category is required', required)},
      //   ...this.formFields[this.form.category].reduce((acc, field) => {
      //     acc[field] = {required};
      //     return acc;
      //   }, {}),
      },
    }
  },
});

</script>


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