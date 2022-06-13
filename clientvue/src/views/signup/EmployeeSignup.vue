<template>
  <div class="background">
    <div class="container">
        <h2>KerjaSini</h2>
        <h4>Create an account</h4>
        <p>Phone Number</p>
        <input type="text" v-model="phonenumber">
        <p>Full Name</p>
        <input type="text" v-model="fullname">
        <p>Username</p>
        <input type="text" v-model="username">
        <p>Email</p>
        <input type="email" v-model="email">
        <p>Age</p>
        <input type="text" v-model="age">
        <p>Postcode</p>
        <input type="text" v-model="postcode">
        <p>New Password</p>
        <input type="password" v-model="pass">
        <p>Please Retype Password</p>
        <input type="password" v-model="retype_pass">
        <button @click="idk"><span>Sign Up as Job Seeker</span></button>
        <button @click="$router.push('mainlogin')"><span>Login Page</span></button>
    </div>
  </div>
</template>

<script>
export default {
    data() {
        return {
            phonenumber : '',
            fullname: '',
            username: '',
            email: '',
            age: '',
            pass: '',
            salt: '',
            retype_pass: '',
            postcode: '',
            retype_err: false,
            pass_err: false,
            phonenumber_taken: false,
            email_err: false,
            age_err: false,
            postcode_err: false
        }
    },
}
</script>

<style scoped>
.background {
    position:absolute;
    top:0px;
    right:0px;
    bottom:0px;
    left:0px;
    background: -webkit-linear-gradient(to right, #acb6e5, #86fde8); /* Chrome 10-25, Safari 5.1-6 */
    background: linear-gradient(to right, #acb6e5, #86fde8);
    display: flex;
    justify-content: center;
    align-items: center;
}

.container {
    display: grid;
    place-items: right;
    background-color: rgb(254, 252, 252);
    margin-top: 7%;
    margin-bottom:  7%;
    margin-right: 20%;
    margin-left: 20%;
    min-height: 80%;
    min-width: 30%;
    border-radius: 10px;
    padding-bottom: 10px;
    padding-left: 2%;
    padding-right: 2%;
}

button {
    width: 50%;
    margin-top: 5px;
    margin-bottom: 0px;
}

p {
    margin-bottom: 3px;
}

h4 {
    margin-bottom: 5px;
}

h2 {
    margin-bottom: 0px;
}

</style>