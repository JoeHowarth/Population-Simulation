/*eslint no-unused-vars: "off"*/
import Vue from 'vue';
import './plugins/vuetify';
import App from './App.vue';
import router from './router';
import store from './store';
import { renderMapGL } from "./map_gen/render/render-map";
import { setup_canvas } from "./map_gen/render/webgl";
document.addEventListener('DOMContentLoaded', () => {
    Vue.config.productionTip = false;
    new Vue({
        router,
        store,
        render: h => h(App)
    }).$mount('#app');
}, false);
export function main(mesh) {
    console.log("main", mesh);
    setup_canvas(mesh.Dkm[0], mesh.Dkm[1]);
    renderMapGL(mesh, mesh.h);
}
//# sourceMappingURL=main.js.map