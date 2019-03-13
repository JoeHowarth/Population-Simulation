import VueNativeSock from 'vue-native-websocket'
import store from './store'
import Vue from 'vue'

const url = 'ws://127.0.0.1:8090';
Vue.use(VueNativeSock, url, {
  format: 'json',
  // reconnection : true,
  // reconnectionAttemps: 5,
  // reconnectionDelay: 1000,
})

const vm = new Vue()

vm.$options.sockets.onmessage = (msg) => {
  console.log(msg)
  let data = JSON.parse(msg.data)

  if (data.mutation) {
    store.commit(data.mutation, data.inner)
  }
}


