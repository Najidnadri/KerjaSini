<template>
  <div class = "loginform">
    <h2>
      WELCOME! {{phoneNumber}}
    </h2>
      <p>Phone Number</p>
      <input type="text" v-model="phoneNumber">
      <p>Password</p>
      <input type="password" v-model="password">
      <button onclick="#">
      <span>EMPLOYER LOGIN</span>
      </button>
    <button @click = "$parent.employerLogin = false">
      <span>BACK</span>
    </button>
    <router-link to="/employersignup">Sign up here</router-link>
  </div>
</template>

<script>
import { ref } from '@vue/reactivity';
export default {
    setup() {
        const phoneNumber = ref('');
        const password = '';
        return {phoneNumber, password}
    }
}
</script>

<style scoped>
.loginform {
  min-width: 300px;
  padding-right: 5px;
}

input {
  min-width: 100%;
}

button {
  font-family: -apple-system,system-ui,"Segoe UI",Roboto,"Helvetica Neue",Ubuntu,sans-serif;
  align-items: center;
  background: -webkit-linear-gradient(to right, #acb6e5, #86fde8); /* Chrome 10-25, Safari 5.1-6 */
  background: linear-gradient(to right, #acb6e5, #86fde8);
  border: 0;
  border-radius: 3px;
  box-shadow: rgba(151, 65, 252, 0.2) 0 15px 30px -5px;
  box-sizing:border-box;
  color: #FFFFFF;
  font-size: 90%;
  font-weight: 500;
  justify-content: center;
  line-height: 1em;
  width: 100%;
  padding: 3px;
  text-decoration: none;
  user-select: none;
  -webkit-user-select: none;
  touch-action: manipulation;
  white-space: nowrap;
  cursor: pointer;
  margin-top: 5px;
}

button:active,
button:hover {
  outline: 0;
  color: black;
}


button span {
  background-color: rgb(33, 32, 32);
  padding: 8%;
  border-radius: 3px;
  width: 100%;
  transition: 300ms;
  display: flex;
  justify-content: center;
  align-items: center;
  font-family: 'Courier New',monospace, -apple-system,system-ui,"Segoe UI",Roboto,"Helvetica Neue",Ubuntu,sans-serif;
}

button:hover span {
  background: none;
}

@media (min-width: 768px) {
  .button {
    font-size: 24px;
    min-width: 196px;
  }
}
</style>



