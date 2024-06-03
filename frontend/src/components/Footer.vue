<template>
    <footer>

        <div>
            <router-link to="/home">© Adee 2023</router-link>
            <router-link to="/tac">Terms And Conditions</router-link>
        </div>
        
        
        
    </footer>
</template>

<style scoped>

footer {
    height: 50px;
    display: flex;
    justify-content: center;
    align-items: center;
}

div {
    display: flex;
    justify-content: center;   
    align-items: center;
    flex-direction: row;
    gap:40px;
}

</style>