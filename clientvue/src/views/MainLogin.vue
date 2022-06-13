<template>
    <div class="background">
        <div class="container">
            <section class="column">
                <div class="left-column">
                    <EmployeeLoginForm v-if="employeeLogin" />
                    <EmployerLoginForm v-else-if="employerLogin" />
                    <div v-else>
                        <div class="choice">
                            <h2>Welcome!</h2>
                            <button @click="employeeLogin = !employeeLogin">
                                <span>
                                    LOGIN AS JOB SEEKER
                                </span>
                            </button>
                            <button @click="employerLogin = !employerLogin">
                                <span>
                                    LOGIN AS EMPLOYER
                                </span>
                            </button>
                        </div>
                    </div>
                </div>
            </section>
            <section class="column">
                <div class="right-column">
                    <h2>HARD WORK BEATS TALENT WHEN TALENT DOESN'T WORK HARD</h2>
                    <p>TIM NOTKE</p>
                </div>
            </section>
        </div>
    </div>
</template>

<script>
import EmployeeLoginForm from "./login/EmployeeLoginForm.vue";
import EmployerLoginForm from "./login/EmployerLoginForm.vue";

export default {
    components: {EmployeeLoginForm, EmployerLoginForm},
    data() {
        return {
            employerLogin: false,
            employeeLogin: false,
        }
    }
}
</script>


<style scoped>

.choice {
    min-width: 300px;
    padding-right: 20px;
}

h2 {
    padding-left: 20px;
}

.hide {
    visibility: hidden;
}

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
    grid-template-columns: 60% 40%;
    place-items: center;
    background-color: rgb(254, 252, 252);
    margin-top: 7%;
    margin-bottom:  7%;
    margin-right: 20%;
    margin-left: 20%;
    min-height: 80%;
    border-radius: 10px;
}

.column {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 50%;
    width: 50%;
    text-align: center;
}

.left-column {
    color: rgb(72, 72, 72);
    font-family: -apple-system,system-ui,"Segoe UI",Roboto,"Helvetica Neue",Ubuntu,sans-serif;
}

.right-column {
    color: rgb(72, 72, 72);
    display: grid;
    grid-template-rows: 1fr, 1fr;
    font-family: -apple-system,system-ui,"Segoe UI",Roboto,"Helvetica Neue",Ubuntu,sans-serif;
}

button {
    margin: 10px;
}

</style>