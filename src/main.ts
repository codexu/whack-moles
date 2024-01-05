import { createApp } from "vue";
import Antd from "ant-design-vue";
import "./styles.css";
import App from "./App.vue";
import 'ant-design-vue/dist/reset.css';

import Home from "./views/home.vue";
import Screenshot from "./views/screenshot.vue";
import { createRouter, createWebHashHistory } from "vue-router";

const routes = [
  { path: "/", component: Home },
  { path: "/screenshot", component: Screenshot },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

createApp(App).use(router).use(Antd).mount("#app");
