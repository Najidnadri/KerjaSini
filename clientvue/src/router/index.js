import { createRouter, createWebHistory } from "vue-router";
import MainLogin from "../views/MainLogin.vue";
import HomeView from "../views/HomeView.vue";
import EmployeeSignup from "../views/signup/EmployeeSignup.vue";
import EmployerSignup from "../views/signup/EmployerSignup.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/mainlogin",
      name: "mainlogin",
      component: MainLogin,
    },
    {
      path: "/employeesignup",
      name: "employeesignup",
      component: EmployeeSignup,
    },
    {
      path: "/employersignup",
      name: "employersignup",
      component: EmployerSignup,
    }
  ],
});

export default router;
