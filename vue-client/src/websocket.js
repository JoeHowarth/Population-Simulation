import VueNativeSock from 'vue-native-websocket'
import store from './store'
import Vue from 'vue'

const url = 'ws://127.0.0.1:8090';
// Vue.use(VueNativeSock, url, {
//   store: store,
//   format: 'json',
  // reconnection: true,
  // reconnectionAttempts: 1000,
  // reconnectionDelay: 5000,
// })
Vue.use(VueNativeSock, url, {
  store:store
})

