import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router';
import Tpl from './Tpl.vue';
import DemandList from './views/DemandList.vue';
import Demand from './views/Demand.vue';
import Ongoing from './views/Ongoing.vue'
import Complate from './views/Complate.vue'
import Deploy from './views/Deploy.vue'
import Report from './views/Report.vue'
import Setting from './views/Setting.vue';
import About from './views/About.vue';
import CSSDoodle from 'css-doodle';

import 'vue3-perfect-scrollbar/dist/vue3-perfect-scrollbar.css';
import 'element-plus/es/components/notification/style/css';

const routes = [
    { path: '/', component: DemandList },
    { path: '/ongoing', component: Ongoing },
    { path: '/complate', component: Complate },
    { path: '/deploy', component: Deploy },
    { path: '/demand/:demandId', component: Demand },
    { path: '/report', component: Report },
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

