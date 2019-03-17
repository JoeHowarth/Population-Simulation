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

  console.log(data)
  if (data.mutation) {
    store.commit(data.mutation, data.inner)
  }
  if (data.SubPush) {
    store.commit("subPush" + data.SubPush.section, data.SubPush);
  }
}


// could do validation of subscription requests, but let's just do it serverside
/*
let validation = new Map([
  ["Agr", new Set(["FarmData", "BaseFarmData", "FoodStore"])],
  ["Terr", new Set(["Region", "Weather", "RiverID", "LandMassID", ])]
])*/

export function subReq(section, component, insert, keys = false) {
  let obj = {
    SubReq: {
      section: section,
      insert,
      component,
    }
  }

  if (keys) {
    obj.keys = keys
  }

  console.log("Sending SubReq: ", obj)
  vm.$socket.sendObj(obj)

}
