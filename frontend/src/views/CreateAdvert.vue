<script>
import { CREATE_ADVERT } from '@/graphql/advert';
import { useMutation } from '@vue/apollo-composable';
import useVuelidate from '@vuelidate/core'
import { useRouter } from 'vue-router';
import { ref } from "vue";
import { required, email, minLength, helpers } from '@vuelidate/validators'


const { withMessage } = helpers



export default {
  setup() {

    const formKeys = ['price', 'location', 'title', 'description', 'category', 'model', 'year', 'fuelType', 'brand', 'condition', 'photoUrl'];
    const router = useRouter();
    const { mutate: createAdvert } = useMutation(CREATE_ADVERT);
    const selectedFile = ref([]);

   

    const accessToken = localStorage.getItem("access_token");
    const enumerations = [
    'Electronics',
    'Cars',
    'Fashion',
    'Home & Garden',
    'Sport & Leisure',
    'Movies, Books & Music',
    'Pets',
    'Services',
    'Other']

    const handleFileUpload = (event) => {
  console.log("files", event.target.files);
  selectedFile.value = event.target.files; // Corrected variable name here
};


    const create = async function() {
        console.log(this.v$.form);

        let formToSend = {};


        formKeys.forEach(key => {
          console.log(key);

          if ( key === 'category' || key === 'price' || key === 'location' || key === 'title' || key === 'description' || key === 'photoUrl') {
            return;
          }

          try {
            formToSend[key] = this.v$.form[key].$model;
          } catch (error) {
            // do nothing
            
          }
        });
        
        console.log(selectedFile.value);

        console.log(formToSend, this.v$.form);

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



        await createAdvert({
          price: parseFloat(this.v$.form.price.$model),
          location: this.v$.form.location.$model,
          title: this.v$.form.title.$model,
          description: this.v$.form.description.$model,
          category: this.v$.form.category.$model,
          data: formToSend,
          photos: urls,
          accessToken
          
        }).then(({ data, loading, error }) => {
          if (error) {
            console.error(`An error occurred: ${error.message}`);
            return;
          }
          console.log(data);
          router.push(`/advert/${data.createAdvert.id}`)
         });
  
      }

    return {  create, v$: useVuelidate(), enumerations, handleFileUpload}
  },
  data() {
    return {
      form: {
        price: 0.0,
        location: '',
        title: '',
        description: '',
        category: '',
      },
      formFields: {
        '': [],
        'Cars': ['model', 'year', 'fuelType'],
        'Electronics': ['brand', 'model', 'condition'],
        // Add other categories here
      },
    }
  },
  computed: {
    selectedCategoryFields() {
      return this.formFields[this.form.category];
    }
  },
  validations() {
    
    return {
      form: {
        price: {
          required: withMessage('Price is required', required),
        },
        location: {required: withMessage('Surname is required', required)},
        title: {required: withMessage('title is required', required)},
        description: {required: withMessage('description is required', required)},
        category: {required: withMessage('category is required', required)},
        ...this.formFields[this.form.category].reduce((acc, field) => {
          acc[field] = {required};
          return acc;
        }, {}),
      },
      
    }
  },
}
</script>

<template>
    <div class="title"><h1>Create Advert:</h1></div>
    <div class="fields">
      <div class="input">
      <div class="input-field">
        <input type="number" name="" id="price" v-model="v$.form.price.$model" placeholder="Price">
      </div>
      
      <div class="input-field">
        <input type="text" name="" id="location" v-model="v$.form.location.$model" placeholder="Location" maxlength="10">
      </div>

      <div class="input-field">
        <input type="text" name="" id="title" v-model="v$.form.title.$model" placeholder="Title" maxlength="20">
      </div>

      
   
      
    </div>
    <div class="input-field category">
      Category:
        <select v-model="v$.form.category.$model">
          <option v-for="category in enumerations" :key="category" :value="category">
            {{ category }}
          </option>
        </select>
    </div>
    <div class="input second">
      <div class="input-field" v-for="field in selectedCategoryFields" :key="field">
        <input type="text" v-model="form[field]" :placeholder="field" maxlength="15">
        <div v-if="v$.form[field].$error">
          This field is required.
        </div>
      </div>
    </div>
    <textarea id="freeform" name="freeform" rows="4" cols="50" v-model="v$.form.description.$model">
      Description
    </textarea>
    <div class="input-field last">
        <label for="photos">Photos:</label>
        <input type="file" accept="image/*" id="photos" multiple @change="handleFileUpload($event)"  />
      </div>
      <span class="error" v-if="v$.form.price.$error">Price is required and must be between 0 and 10000.</span>
      <span class="error" v-if="v$.form.location.$error">Location is required.</span>
      <span class="error" v-if="v$.form.title.$error">Title is required.</span>
      




    <button class="press" :disabled="v$.form.$invalid" @click="create">Create</button>
    </div>
    
    
</template>



<style scoped>

@media (max-width: 1400px) {
  h1 {
      text-align: center !important;
      margin: 50px 20px !important;
  }
}

.error {
  color: rgb(var(--v-theme-text));


}

input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
}

#price {
  -webkit-appearance: none;
  -moz-appearance: textfield;
  appearance: none;
  margin: 0; 

}

  .last {
    margin-bottom: 50px;
    max-width: 428px;
    box-sizing: border-box;
  }

  textarea {
    outline: 0;
    padding: 10px 15px;
    max-width:428px;
  }

  select {
    outline: 0;
  }

  .fields {
      display:flex;
      justify-content:flex-start;
      align-items:center;
      flex-direction: column;
      margin:50px 150px;
  }

  .input {
    /* make inputs 2 in a row, unlimited rows */
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-auto-rows: minmax(20px, auto);
    grid-gap: 20px;

  }

  option {
    background-color:rgb(var(--v-theme-background));
    color: rgb(var(--v-theme-inputText))
  }

  .category {
    margin-top:50px;
  }

  .input-field {
    height:40px;
    padding: 10px 10px;
    border-radius: 50px;
    background-color: transparent;
    border: 1px solid  rgba(var(--v-theme-inputText), 0.3);
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: row;
    transition: color 0s ease;
  }

  .title {
    display:grid;
  }

  h1 {
    color: rgb(var(--v-theme-text));
    margin: 50px 150px 20px 150px;
    justify-self: flex-start;
  }

  input {
   color: rgba(var(--v-theme-inputText), 0.87);
   outline: none;
  }

  .second {
    margin-top:50px;
    margin-bottom: 50px;

  }

  input::placeholder {
    color: rgba(var(--v-theme-inputText), 0.3);

  }

.press {
   padding: 10px 60px;
   border-radius: 50px;
   color: rgb(var(--v-theme-background));
}

textarea {
  margin-bottom: 50px;
  background-color: transparent;
    border: 1px solid  rgba(var(--v-theme-inputText), 0.3);
}

</style>