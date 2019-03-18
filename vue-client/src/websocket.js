import VueNativeSock from 'vue-native-websocket'
import store from './store'
import Vue from 'vue'

const url = 'ws://127.0.0.1:8090';
// Vue.use(VueNativeSock, url, {
//   format: 'json',
  // reconnection : true,
  // reconnectionAttemps: 5,
  // reconnectionDelay: 1000,
// })

let vm = new Vue()

vm.$socket = new WebSocket(url)
Vue.prototype.$socket = vm.$socket
vm.$socket.subReq = subReq

vm.$socket.addEventListener("open", () => {
  store.commit("socketOnOpen")
  // request date subscription
  // subReq("Time", "Date", true);
})

vm.$socket.sendObj = (obj) => {
  const str = JSON.stringify(obj)
  vm.$socket.send(str)
}

vm.$socket.addEventListener("message", function (msg) {
  // console.log(msg)
  let data = JSON.parse(msg.data)


  if (data.mutation) {
    store.commit(data.mutation, data.inner)
  }
  if (data.SubPush) {
    store.commit("subPush" + data.SubPush.section, data.SubPush);
  }
})


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
  store.commit("updateActiveSubs", obj.SubReq)
  if (!insert) {
    store.commit("delData" + section, {section, component})
  }

  console.log("Sending SubReq: ", obj)
  vm.$socket.sendObj(obj)

}

