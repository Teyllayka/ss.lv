<template> 
  <div class="title"><h1>Contact Us:</h1></div>
  <div class="container">
    <div class="contact">
      <p>Contact form</p>
      <div class="first"><input type="text" placeholder="Name"> <input type="text" name="" id="" placeholder="Email"></div>
      <div class="phone"><input type="text" placeholder="Phone number"></div>
      <textarea name="" id="" cols="30" rows="10"></textarea>
      <button>Send</button>

    </div>
  </div>
</template>

<style scoped>

p {
  font-size:40px;
  color: rgb(var(--v-theme-text));
  font-size:500;
}

.first  {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: row;
  gap:20px;
}

.phone {
  width:100%;
}

.phone input {
  width:100%;
}

input {
  color: rgba(var(--v-theme-inputText), 0.87);
  outline: none;
  padding: 10px 15px;
  border-radius: 50px;
  background-color: transparent;
  border: 1px solid  rgba(var(--v-theme-inputText), 0.3);
}

.contact {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  gap:20px;
}

/* .contact:last-child {
  align-items: flex-start;
} */


button {
  padding: 10px 30px;
  border-radius: 50px;
  color: rgb(var(--v-theme-background));
  width:200px;

}

textarea {
    background-color: transparent;
    border: 1px solid  rgba(var(--v-theme-inputText), 0.3);
    width:100%;
}

.container {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
  margin: 50px 150px 20px 150px;
  gap: 20px;
}

.title {
    display:grid;
  }

  h1 {
    color: rgb(var(--v-theme-text));
    margin: 50px 150px 20px 150px;
    justify-self: flex-start;
  }
</style>