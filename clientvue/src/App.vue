<script setup>
import { RouterLink, RouterView } from "vue-router";
</script>

<template>

  <RouterView />
</template>

<style>
 @import './assets/base.css';

button {
  align-items: center;
  background: -webkit-linear-gradient(to right, #acb6e5, #86fde8); /* Chrome 10-25, Safari 5.1-6 */
  background: linear-gradient(to right, #acb6e5, #86fde8);
  border: 0;
  border-radius: 3px;
  box-shadow: rgba(151, 65, 252, 0.2) 0 15px 30px -5px;
  box-sizing: border-box;
  color: #FFFFFF;
  display: flex;
  font-size: 80%;
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
  font-family: -apple-system,system-ui,"Segoe UI",Roboto,"Helvetica Neue",Ubuntu,sans-serif;
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
  font-family: -apple-system,system-ui,"Segoe UI",Roboto,"Helvetica Neue",Ubuntu,sans-serif;
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
