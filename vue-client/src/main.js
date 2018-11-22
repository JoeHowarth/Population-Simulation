/*eslint no-unused-vars: "off"*/
import Vue from 'vue'
import './plugins/vuetify'
import App from './App.vue'
import router from './router'
import websocket from './websocket'
import store from './store'
import mapGen from './map_gen/map_gen'

document.addEventListener('DOMContentLoaded', mapGen, false)

Vue.config.productionTip = false

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
