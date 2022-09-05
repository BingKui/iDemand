import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router';
import Tpl from './Tpl.vue';
import DemandList from './views/DemandList.vue';
import Demand from './views/Demand.vue';
import Complate from './views/Complate.vue'
import Setting from './views/Setting.vue';
import About from './views/About.vue';
import CSSDoodle from 'css-doodle';

import 'vue3-perfect-scrollbar/dist/vue3-perfect-scrollbar.css';
import 'element-plus/es/components/notification/style/css';

const routes = [
    { path: '/', component: DemandList },
    { path: '/complate', component: Complate },
    { path: '/demand/:demandId', component: Demand },
    { path: '/setting', component: Setting },
    { path: '/about', component: About },
];

const router = createRouter({
    history: createWebHashHistory(),
    routes,
});

const app = createApp(Tpl);

app.use(CSSDoodle);
app.use(router);

app.mount('#app')

