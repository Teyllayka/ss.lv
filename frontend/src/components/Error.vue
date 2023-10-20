<template>
    <div>
        sperma {{ message }} sperma
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