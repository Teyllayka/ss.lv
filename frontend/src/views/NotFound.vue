<template>
    <div>
        <p>404 NOT FOUND</p>
        <router-link to="/home">Back To Home</router-link>
       
    </div>

</template>

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