<template>
    <div>
        <p>An error has occurred</p>
        <p>{{ message }}</p>
        <router-link to="/home">Back To Home</router-link>
       
    </div>

</template>

<script lang="ts">
import { defineComponent } from 'vue';


export default defineComponent({
    name: 'App',
    props: ['message'],
    setup(props) {
        console.log(props);
        return {
        }
    }


})

</script>



<style scoped>

div {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
}



p {
    font-size: 40px;
    color: rgba(var(--v-theme-inputText), 0.87);
}

a {
    font-size:20px;
}
</style>