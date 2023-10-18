<script>
import { CREATE_ADVERT } from '@/graphql/advert';
import { useMutation } from '@vue/apollo-composable';
import useVuelidate from '@vuelidate/core'
import { required } from '@vuelidate/validators'
import { useRouter } from 'vue-router';



export default {
  setup() {

    const formKeys = ['price', 'location', 'title', 'description', 'category', 'model', 'year', 'fuelType', 'brand', 'condition'];
    const router = useRouter();
    const { mutate: createAdvert } = useMutation(CREATE_ADVERT);
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


    const create = async function() {
        console.log(this.v$.form);

        let formToSend = {};


        formKeys.forEach(key => {
          console.log(key);

          try {
            formToSend[key] = this.v$.form[key].$model;
          } catch (error) {
            // do nothing
            
          }
        });

        console.log(formToSend);


        await createAdvert({
          price: parseFloat(this.v$.form.price.$model),
          location: this.v$.form.location.$model,
          title: this.v$.form.title.$model,
          description: this.v$.form.description.$model,
          category: this.v$.form.category.$model,
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

    return {  create, v$: useVuelidate(), enumerations }
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
        price: {required},
        location: {required},
        title: {required},
        description: {required},
        category: {required},
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
    <div>create advert</div>
    <div class="input-field">
        <input type="text" name="" id="text" v-model="v$.form.price.$model" placeholder="Price">
    </div>
    <div class="input-field">
        <input type="text" name="" id="location" v-model="v$.form.location.$model" placeholder="Location">
    </div>
    <div class="input-field">
        <input type="text" name="" id="title" v-model="v$.form.title.$model" placeholder="Title">
    </div>
    <div class="input-field">
        <input type="text" name="" id="description" v-model="v$.form.description.$model" placeholder="Description">
    </div>
    <div class="input-field">
      <select v-model="v$.form.category.$model">
        <option v-for="category in enumerations" :key="category" :value="category">
          {{ category }}
        </option>
      </select>
    </div>
    <div class="input-field" v-for="field in selectedCategoryFields" :key="field">
      <input type="text" v-model="form[field]" :placeholder="field">
      <div v-if="v$.form[field].$error">
        This field is required.
      </div>
    </div>
    <button class="press" :disabled="v$.form.$invalid" @click="create">Login</button>
</template>

